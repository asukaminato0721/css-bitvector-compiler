// No npm packages are required. Use Node.js with the built-in WebSocket client.
import { readFile, writeFile, mkdtemp, rm } from 'node:fs/promises';
import { spawn } from 'node:child_process';
import { tmpdir, cpus } from 'node:os';
import { join } from 'node:path';
import { createHash } from 'node:crypto';

const args = process.argv.slice(2);
const option = (name, fallback) => {
  const at = args.indexOf(name);
  return at < 0 ? fallback : args[at + 1];
};

function fixtures() {
  const result = [];
  for (const count of [256, 1024]) {
    for (const kind of ['local', 'ancestor', 'sibling', 'position']) {
      const selector = i => ({
        local: `.root > .c${i}.on`,
        ancestor: `.root.active .c${i}`,
        sibling: `.on + .c${i}`,
        position: `.root > .c${i}:nth-child(2n)`,
      })[kind];
      const css = Array.from({ length: 32 }, (_, i) =>
        `${selector(i)} { background-color: rgb(${i + 1}, 80, 160); }`).join('\n');
      const root = { id: 1, name: 'div', attributes: { class: 'root' },
        children: Array.from({ length: count }, (_, i) => ({
          id: i + 2, name: 'div', attributes: { class: `c${i % 32}` }, children: [],
        })) };
      const frames = [{ name: 'init', node: root }];
      for (let i = 0; i < 100; i++) {
        if (kind === 'position') {
          frames.push(i % 2 === 0
            ? { name: 'add', path: [0], node: { id: 100000 + i, name: 'div', attributes: { class: 'c0' }, children: [] } }
            : { name: 'remove', path: [0] });
        } else {
          const index = (Math.floor(i / 2) * 37) % count;
          const base = kind === 'ancestor' ? 'root' : `c${index % 32}`;
          const added = kind === 'ancestor' ? 'active' : 'on';
          frames.push({ name: 'replace_value', type: 'attributes',
            path: kind === 'ancestor' ? [] : [index], key: 'class',
            value: i % 2 === 0 ? `${base} ${added}` : base });
        }
      }
      result.push({ name: `${kind}-${count}`, nodes: count + 1, css, frames });
    }
  }
  return result;
}

// This function runs in the page. It uses real DOM operations and real CSS rules.
function installPage(fixture) {
  let root;
  let ids;
  let pending;
  const sheet = document.createElement('style');
  sheet.textContent = fixture.css;
  document.head.append(sheet);
  function build(node) {
    const element = document.createElement(node.name);
    ids.set(element, node.id);
    for (const [name, value] of Object.entries(node.attributes)) element.setAttribute(name, String(value));
    for (const child of node.children) element.append(build(child));
    return element;
  }
  function at(path) {
    let element = root;
    for (const index of path) element = element.children[index];
    if (!element) throw new Error(`invalid path ${path}`);
    return element;
  }
  function apply(frame) {
    if (frame.name === 'init') {
      root = build(frame.node);
      document.body.append(root);
      pending = root;
    } else if (frame.name === 'replace_value') {
      pending = at(frame.path);
      pending.setAttribute(frame.key, String(frame.value));
    } else if (frame.name === 'add') {
      const parent = at(frame.path.slice(0, -1));
      pending = build(frame.node);
      parent.insertBefore(pending, parent.children[frame.path.at(-1)] ?? null);
    } else if (frame.name === 'remove') {
      const removed = at(frame.path);
      pending = removed.nextElementSibling ?? removed.parentElement;
      removed.remove();
    } else throw new Error(`unsupported frame ${frame.name}`);
  }
  function flush() {
    // A non-layout-dependent property forces pending style updates.
    // Body alone can remain clean while one of its descendants is dirty.
    for (const element of [pending, pending.firstElementChild, pending.nextElementSibling]) {
      if (element) globalThis.styleSink = getComputedStyle(element).backgroundColor;
    }
  }
  function reset() {
    root?.remove();
    root = undefined;
    ids = new WeakMap();
    pending = document.body;
    flush();
  }
  function check(frameIndex) {
    const expected = fixture.expected[frameIndex];
    const colours = new Map();
    for (const selector of fixture.selectors) {
      const wanted = expected[selector] ?? [];
      const elements = [...root.querySelectorAll(selector)];
      if (root.matches(selector)) elements.push(root);
      const actual = elements.map(e => ids.get(e)).sort((a, b) => a - b);
      if (JSON.stringify(actual) !== JSON.stringify(wanted)) {
        throw new Error(`${fixture.name} frame ${frameIndex}: match mismatch for ${selector}`);
      }
      const colour = `rgb(${Number(selector.match(/\.c(\d+)/)[1]) + 1}, 80, 160)`;
      for (const id of wanted) colours.set(id, colour);
    }
    // Check the cascade result as well as Element.matches/querySelectorAll.
    for (const element of [root, ...root.querySelectorAll('*')]) {
      const expectedColour = colours.get(ids.get(element)) ?? 'rgba(0, 0, 0, 0)';
      if (getComputedStyle(element).backgroundColor !== expectedColour) {
        throw new Error(`${fixture.name} frame ${frameIndex}: computed style mismatch`);
      }
    }
  }
  globalThis.benchReset = reset;
  globalThis.benchRun = (workload, validate = false) => {
    const selected = workload === 'initial' ? fixture.frames.slice(0, 1) : fixture.frames;
    const start = performance.now();
    for (let i = 0; i < selected.length; i++) {
      apply(selected[i]);
      flush();
      if (validate) check(i);
    }
    return { elapsed_ms: performance.now() - start, frames: selected.length };
  };
  reset();
}

class Protocol {
  constructor(socket) {
    this.socket = socket;
    this.next = 0;
    this.pending = new Map();
    socket.addEventListener('message', event => {
      const reply = JSON.parse(event.data);
      if (!reply.id) return;
      const pending = this.pending.get(reply.id);
      this.pending.delete(reply.id);
      if (reply.error) pending.reject(new Error(JSON.stringify(reply.error)));
      else pending.resolve(reply.result);
    });
    socket.addEventListener('close', () => {
      for (const pending of this.pending.values()) pending.reject(new Error('browser connection closed'));
      this.pending.clear();
    });
  }
  send(method, params = {}, sessionId) {
    const id = ++this.next;
    return new Promise((resolve, reject) => {
      this.pending.set(id, { resolve, reject });
      this.socket.send(JSON.stringify({ id, method, params, sessionId }));
    });
  }
}

async function benchmark() {
  const input = option('--oracle');
  const output = option('--output', 'docs/browser-results.json');
  const samples = Number(option('--samples', '7'));
  if (!input || samples < 1 || samples % 2 !== 1) throw new Error('use --oracle <file> and a positive odd --samples');
  const data = JSON.parse(await readFile(input, 'utf8'));
  const profile = await mkdtemp(join(tmpdir(), 'css-blink-'));
  const executable = option('--browser', '/usr/bin/brave');
  const browserArgs = ['--headless', '--disable-gpu', '--no-first-run', '--no-default-browser-check',
    '--disable-background-networking', '--disable-component-update', '--disable-sync', '--disable-extensions',
    '--remote-debugging-port=0', `--user-data-dir=${profile}`, 'about:blank'];
  const child = spawn(executable, browserArgs, { stdio: ['ignore', 'ignore', 'pipe'] });
  let client;
  try {
    const url = await new Promise((resolve, reject) => {
      let log = '';
      const timeout = setTimeout(() => reject(new Error(`browser startup timeout: ${log.slice(-2000)}`)), 30000);
      child.on('error', error => { clearTimeout(timeout); reject(error); });
      child.on('exit', code => { clearTimeout(timeout); reject(new Error(`browser exited ${code}: ${log.slice(-2000)}`)); });
      child.stderr.on('data', chunk => {
        log += chunk;
        const match = log.match(/DevTools listening on (ws:\/\/\S+)/);
        if (match) { clearTimeout(timeout); resolve(match[1]); }
      });
    });
    const socket = new WebSocket(url);
    await new Promise((resolve, reject) => {
      socket.addEventListener('open', resolve, { once: true });
      socket.addEventListener('error', reject, { once: true });
    });
    client = new Protocol(socket);
    const version = await client.send('Browser.getVersion');
    const report = { captured_at_utc: new Date().toISOString(), version, executable, browser_args: browserArgs,
      cpu: cpus()[0].model, cpu_affinity: (await readFile('/proc/self/status', 'utf8')).match(/^Cpus_allowed_list:.*$/m)[0],
      harness_sha256: createHash('sha256').update(await readFile(import.meta.filename)).digest('hex'),
      samples, warmups: 1, timer: 'performance.now; CDP Performance with timeTicks', results: [] };
    for (const fixture of data) {
      const { targetId } = await client.send('Target.createTarget', { url: 'about:blank' });
      const { sessionId } = await client.send('Target.attachToTarget', { targetId, flatten: true });
      const call = (method, params = {}) => client.send(method, params, sessionId);
      const evaluate = async expression => {
        const reply = await call('Runtime.evaluate', { expression, returnByValue: true });
        if (reply.exceptionDetails) throw new Error(JSON.stringify(reply.exceptionDetails));
        return reply.result.value;
      };
      await call('Performance.enable', { timeDomain: 'timeTicks' });
      await evaluate('document.open(); document.write("<!doctype html><html><head></head><body></body></html>"); document.close()');
      await evaluate(`(${installPage.toString()})(${JSON.stringify(fixture)})`);
      const validationBefore = (await call('Performance.getMetrics')).metrics.find(x => x.name === 'RecalcStyleCount').value;
      await evaluate('benchRun("trace", true)');
      const validationAfter = (await call('Performance.getMetrics')).metrics.find(x => x.name === 'RecalcStyleCount').value;
      const validationStyleCount = validationAfter - validationBefore;
      // The full computed-style check must not trigger extra deferred passes.
      if (validationStyleCount !== fixture.frames.length) {
        throw new Error(`validation needed ${validationStyleCount} style passes for ${fixture.frames.length} frames`);
      }
      console.log(`${fixture.name}: exact matches and computed styles passed at all ${fixture.frames.length} frames`);
      const scenarios = [];
      for (const workload of ['initial', 'trace']) {
        const values = [];
        for (let round = 0; round <= samples; round++) {
          await evaluate('benchReset()');
          const before = Object.fromEntries((await call('Performance.getMetrics')).metrics.map(x => [x.name, x.value]));
          const measured = await evaluate(`benchRun(${JSON.stringify(workload)})`);
          const after = Object.fromEntries((await call('Performance.getMetrics')).metrics.map(x => [x.name, x.value]));
          for (const name of ['RecalcStyleDuration', 'RecalcStyleCount', 'LayoutDuration']) {
            if (!(name in before) || !(name in after)) throw new Error(`missing CDP metric ${name}`);
          }
          const value = { elapsed_ns: Math.round(measured.elapsed_ms * 1e6),
            style_ns: Math.round((after.RecalcStyleDuration - before.RecalcStyleDuration) * 1e9),
            style_count: after.RecalcStyleCount - before.RecalcStyleCount,
            layout_ns: Math.round((after.LayoutDuration - before.LayoutDuration) * 1e9) };
          if (value.style_count < measured.frames) {
            throw new Error(`only ${value.style_count} style updates for ${measured.frames} frames`);
          }
          if (round > 0) values.push(value);
          console.log(`${fixture.name} ${workload}: browser round ${round}/${samples}`);
        }
        const median = field => values.map(x => x[field]).sort((a, b) => a - b)[Math.floor(samples / 2)];
        scenarios.push({ workload, samples: values, median_ns: median('elapsed_ns'),
          median_style_ns: median('style_ns'), median_style_count: median('style_count'), median_layout_ns: median('layout_ns') });
      }
      report.results.push({ name: fixture.name, nodes: fixture.nodes, selectors: fixture.selectors.length,
        frames: fixture.frames.length, checked_frames: fixture.frames.length,
        validation_style_count: validationStyleCount, scenarios });
      await writeFile(output, JSON.stringify(report, null, 2) + '\n');
      await client.send('Target.closeTarget', { targetId });
    }
  } finally {
    if (client) {
      await client.send('Browser.close').catch(() => {});
      client.socket.close();
    }
    child.kill('SIGTERM');
    await new Promise(resolve => child.exitCode !== null || child.signalCode !== null
      ? resolve() : child.once('exit', resolve));
    await rm(profile, { recursive: true, force: true });
  }
}

if (args.includes('--generate')) {
  await writeFile(option('--generate'), JSON.stringify(fixtures()));
} else {
  await benchmark();
}
