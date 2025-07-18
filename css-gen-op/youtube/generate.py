import json
import sys
from typing import Any, Dict, List, Optional, Set, TextIO

from common import *

TOTAL_DIFF_SIZE: int = 0
TOTAL_SIZE: int = 0


def report_diff(x: Dict[str, Any]) -> None:
    global TOTAL_DIFF_SIZE
    TOTAL_DIFF_SIZE += size(x)


trace_path: str = sys.argv[1]

output: TextIO = open("command.json", "w")


def out(x: Dict[str, Any]) -> None:
    output.write(json.dumps(x) + "\n")


class Counter:
    def __init__(self) -> None:
        self.cnt: int = 0

    def count(self) -> None:
        self.cnt += 1

    def num(self) -> int:
        return self.cnt


INSERT_COUNT: Counter = Counter()
REMOVE_COUNT: Counter = Counter()
REPLACE_COUNT: Counter = Counter()
REPLACE_VALUE_COUNT: Counter = Counter()
INSERT_VALUE_COUNT: Counter = Counter()
DELETE_VALUE_COUNT: Counter = Counter()


def diff_simple_dict(
    l_dict: Dict[str, Any], r_dict: Dict[str, Any], path: List[int], on: str, type_: str
) -> None:
    for k in l_dict.keys():
        if k in r_dict:
            if l_dict[k] != r_dict[k]:
                REPLACE_VALUE_COUNT.count()
                out(command_replace_value(path, on, type_, k, l_dict[k], r_dict[k]))
        else:
            DELETE_VALUE_COUNT.count()
            out(command_delete_value(path, on, type_, k, l_dict[k]))
    for k in r_dict.keys():
        if k not in l_dict:
            INSERT_VALUE_COUNT.count()
            out(command_insert_value(path, on, type_, k, r_dict[k]))


# tree diffing is very hard.
# one possible road is to use difftastic, but conversion between our stuff and theirs is also very hard.
# luckily the diffs is pretty trivial, and we have id to rematch trees.
def diff_dom_tree(lhs: Dict[str, Any], rhs: Dict[str, Any], path: List[int]) -> None:
    lhs_id: Optional[Any] = lhs.get("id", None)
    rhs_id: Optional[Any] = rhs.get("id", None)
    if lhs_id != rhs_id:
        report_diff(rhs)
        REPLACE_COUNT.count()
        out(command_replace(path, lhs, rhs))
    else:
        # Create a safer node identifier instead of truncating the full str representation
        node_identifier = f"{{id:{lhs.get('id', 'None')}, name:'{lhs.get('name', '')}', type:'{lhs.get('type', '')}'}}"

        if lhs["attributes"] != rhs["attributes"]:
            diff_simple_dict(
                lhs["attributes"],
                rhs["attributes"],
                path,
                node_identifier,
                "attributes",
            )
        l_children: List[Dict[str, Any]] = lhs["children"]
        r_children: List[Dict[str, Any]] = rhs["children"]

        l_ids: List[Optional[Any]] = list(x.get("id", None) for x in l_children)
        r_ids: List[Optional[Any]] = list(x.get("id", None) for x in r_children)
        unused_l_i: int = 0
        # invariant: elements with indexe before r_i have been fixed.
        # fixing them consume everything before unused_l_id.
        for r_i in range(len(r_ids)):
            r_id: Optional[Any] = r_ids[r_i]
            found: bool = False
            for l_i in range(unused_l_i, len(l_ids)):
                if l_ids[l_i] == r_id:
                    assert not found
                    for x in range(l_i - unused_l_i):
                        REMOVE_COUNT.count()
                        out(command_remove(path + [r_i], l_children[unused_l_i + x]))
                    diff_dom_tree(l_children[l_i], r_children[r_i], path + [r_i])
                    unused_l_i = l_i + 1
                    found = True
                    break
            if not found:
                INSERT_COUNT.count()
                out(command_add(path + [r_i], r_children[r_i]))


def layout_info(node: Dict[str, Any]) -> Dict[str, Any]:
    key: List[str] = ["type", "x", "y", "width", "height"]
    return {k: node[k] for k in key}


def diff_layout_tree(lhs: Dict[str, Any], rhs: Dict[str, Any], path: List[int]) -> None:
    if layout_info(lhs) != layout_info(rhs):
        out(command_layout_info_changed(path, layout_info(lhs), layout_info(rhs)))
    l_children: List[Dict[str, Any]] = lhs["children"]
    r_children: List[Dict[str, Any]] = rhs["children"]
    if len(l_children) > len(r_children):
        extra: List[Dict[str, Any]] = list(l_children[len(r_children) :])
        for i in range(len(extra)):
            out(
                command_layout_remove(
                    path + [len(l_children) - 1 - i],
                    l_children[len(l_children) - 1 - i],
                )
            )
    elif len(l_children) < len(r_children):
        extra = list(r_children[len(l_children) :])
        for i in range(len(extra)):
            out(command_layout_add(path + [len(l_children) + i], extra[i]))
    for i in range(min(len(l_children), len(r_children))):
        diff_layout_tree(l_children[i], r_children[i], path + [i])


def semantic_check(j: Dict[str, Any]) -> None:
    enforce_unique_id(j, set())


def enforce_unique_id(j: Dict[str, Any], s: Set[Any]) -> None:
    if "id" in j:
        assert j["id"] not in s
        s.add(j["id"])
    if "children" in j:
        for c in j["children"]:
            enforce_unique_id(c, s)


with open(trace_path) as f:
    dom_tree_old: Optional[Dict[str, Any]] = None
    layout_tree_old: Optional[Dict[str, Any]] = None
    for l in f.readlines():
        j: Dict[str, Any] = json.loads(l)
        dom_tree: Optional[Dict[str, Any]] = regularize_dom(j["dom_tree"])
        layout_tree: Dict[str, Any] = regularize_layout(j["layout_tree"])
        if dom_tree is not None:
            semantic_check(dom_tree)
            TOTAL_SIZE += size(dom_tree)
        if dom_tree_old is None:
            assert layout_tree_old is None
            if dom_tree is not None:
                out(command_init(dom_tree, j["time"]))
            out(command_layout_init(layout_tree))
        else:
            assert layout_tree_old is not None
            if dom_tree is not None:
                diff_dom_tree(dom_tree_old, dom_tree, [])
            diff_layout_tree(layout_tree_old, layout_tree, [])
            out(command_recalculate(j["time"]))
        dom_tree_old = dom_tree
        layout_tree_old = layout_tree

print((TOTAL_DIFF_SIZE, TOTAL_SIZE))

print(
    (
        INSERT_COUNT.num(),
        REMOVE_COUNT.num(),
        REPLACE_COUNT.num(),
        REPLACE_VALUE_COUNT.num(),
        INSERT_VALUE_COUNT.num(),
        DELETE_VALUE_COUNT.num(),
    )
)
