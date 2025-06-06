#!/usr/bin/env python3
"""
Incremental CSS Matching Engine
增量式CSS匹配引擎 - 在DOM变化时高效重新计算CSS匹配
"""

import json
import sys
import re
import os
from typing import Dict, Any, List, Set, Optional, Tuple
from common import *


class CSSRule:
    """CSS规则表示"""

    def __init__(self, selector: str, properties: Dict[str, str], priority: int = 0):
        self.selector = selector
        self.properties = properties
        self.priority = priority
        self.compiled_selector = self._compile_selector(selector)

    def _compile_selector(self, selector: str) -> Dict[str, Any]:
        """编译选择器为便于匹配的格式"""
        selector = selector.strip()

        if selector.startswith("#"):
            return {"type": "id", "value": selector[1:]}
        elif selector.startswith("."):
            return {"type": "class", "value": selector[1:]}
        elif " " in selector:
            parts = selector.split()
            return {"type": "descendant", "ancestor": parts[0], "descendant": parts[1]}
        elif ">" in selector:
            parts = [p.strip() for p in selector.split(">")]
            return {"type": "child", "parent": parts[0], "child": parts[1]}
        else:
            return {"type": "tag", "value": selector}


class DOMNode:
    """DOM节点，包含CSS匹配信息"""

    def __init__(self, data: Dict[str, Any]):
        self.id = data.get("id")
        self.name = data.get("name", "")
        self.attributes = data.get("attributes", {})
        self.properties = data.get("properties", {})
        self.children: List[DOMNode] = []
        self.parent: Optional[DOMNode] = None

        self.matched_rules: List[CSSRule] = []
        self.computed_styles: Dict[str, str] = {}
        self.css_dirty = True

        for child_data in data.get("children", []):
            child = DOMNode(child_data)
            child.parent = self
            self.children.append(child)

    def get_classes(self) -> Set[str]:
        """获取节点的所有class"""
        class_attr = self.attributes.get("class", "")
        if isinstance(class_attr, str):
            return set(class_attr.split())
        return set()

    def matches_selector(self, compiled_selector: Dict[str, Any]) -> bool:
        """检查节点是否匹配给定选择器"""
        sel_type = compiled_selector["type"]

        if sel_type == "tag":
            return self.name == compiled_selector["value"]
        elif sel_type == "id":
            return self.attributes.get("id") == compiled_selector["value"]
        elif sel_type == "class":
            return compiled_selector["value"] in self.get_classes()
        elif sel_type == "descendant":
            ancestor_selector = {"type": "tag", "value": compiled_selector["ancestor"]}
            descendant_selector = {
                "type": "tag",
                "value": compiled_selector["descendant"],
            }

            if not self.matches_selector(descendant_selector):
                return False

            current = self.parent
            while current:
                if current.matches_selector(ancestor_selector):
                    return True
                current = current.parent
            return False
        elif sel_type == "child":
            parent_selector = {"type": "tag", "value": compiled_selector["parent"]}
            child_selector = {"type": "tag", "value": compiled_selector["child"]}

            return (
                self.matches_selector(child_selector)
                and self.parent
                and self.parent.matches_selector(parent_selector)
            )

        return False


class IncrementalCSSEngine:
    """增量式CSS匹配引擎"""

    def __init__(self, css_rules: List[CSSRule]):
        self.css_rules = css_rules
        self.dom_root: Optional[DOMNode] = None

    def initialize_dom(self, dom_data: Dict[str, Any]):
        """初始化DOM树"""
        self.dom_root = DOMNode(dom_data)
        self._recalculate_node_css(self.dom_root)
        print(f"🌳 初始化DOM树，根节点: {self.dom_root.name}")

    def _recalculate_node_css(self, node: DOMNode):
        """重新计算节点的CSS匹配"""
        node.matched_rules = []

        for rule in self.css_rules:
            if node.matches_selector(rule.compiled_selector):
                node.matched_rules.append(rule)

        node.computed_styles = {}
        for rule in sorted(node.matched_rules, key=lambda r: r.priority):
            node.computed_styles.update(rule.properties)

        node.css_dirty = False

        for child in node.children:
            self._recalculate_node_css(child)

    def apply_dom_command(self, command: Dict[str, Any]) -> int:
        """应用DOM变化命令，返回受影响的节点数"""
        cmd_name = command.get("name", "")

        if cmd_name == "init":
            self.initialize_dom(command["node"])
            return self._count_nodes(self.dom_root) if self.dom_root else 0

        elif cmd_name == "add":
            return self._handle_add_command(command)
        elif cmd_name == "remove":
            return self._handle_remove_command(command)
        elif cmd_name == "replace":
            return self._handle_replace_command(command)
        elif cmd_name == "replace_value":
            return self._handle_replace_value_command(command)
        elif cmd_name == "insert_value":
            return self._handle_replace_value_command(command)
        elif cmd_name == "delete_value":
            return self._handle_delete_value_command(command)
        elif cmd_name == "recalculate":
            return self._recalculate_all_dirty()

        return 0

    def _find_node_by_path(self, path: List[int]) -> Optional[DOMNode]:
        """根据路径找到节点"""
        if not self.dom_root:
            return None
        if not path:
            return self.dom_root

        current = self.dom_root
        for index in path:
            if index >= len(current.children):
                return None
            current = current.children[index]
        return current

    def _handle_add_command(self, command: Dict[str, Any]) -> int:
        """处理添加节点命令"""
        path = command.get("path", [])
        new_node_data = command.get("node", {})

        if not path:
            return 0

        parent_path = path[:-1]
        insert_index = path[-1]

        parent = self._find_node_by_path(parent_path)
        if not parent:
            return 0

        new_node = DOMNode(new_node_data)
        new_node.parent = parent
        parent.children.insert(insert_index, new_node)

        # 重新计算新节点及其子树
        self._recalculate_node_css(new_node)
        affected_count = self._count_nodes(new_node)

        print(f"➕ 添加节点: {new_node.name} (影响 {affected_count} 个节点)")
        return affected_count

    def _handle_remove_command(self, command: Dict[str, Any]) -> int:
        """处理删除节点命令"""
        path = command.get("path", [])

        if not path:
            return 0

        parent_path = path[:-1]
        remove_index = path[-1]

        parent = self._find_node_by_path(parent_path)
        if not parent or remove_index >= len(parent.children):
            return 0

        removed_node = parent.children.pop(remove_index)
        affected_count = self._count_nodes(removed_node)

        print(f"➖ 删除节点: {removed_node.name} (影响 {affected_count} 个节点)")
        return affected_count

    def _handle_replace_command(self, command: Dict[str, Any]) -> int:
        """处理节点替换命令"""
        path = command.get("path", [])
        new_node_data = command.get("node", {})

        if not path:
            self.initialize_dom(new_node_data)
            return self._count_nodes(self.dom_root) if self.dom_root else 0

        parent_path = path[:-1]
        replace_index = path[-1]

        parent = self._find_node_by_path(parent_path)
        if not parent or replace_index >= len(parent.children):
            return 0

        old_node = parent.children[replace_index]
        new_node = DOMNode(new_node_data)
        new_node.parent = parent
        parent.children[replace_index] = new_node

        self._recalculate_node_css(new_node)
        affected_count = self._count_nodes(new_node)

        print(
            f"🔄 替换节点: {old_node.name} → {new_node.name} (影响 {affected_count} 个节点)"
        )
        return affected_count

    def _handle_replace_value_command(self, command: Dict[str, Any]) -> int:
        """处理属性值替换命令"""
        path = command.get("path", [])
        prop_type = command.get("type", "")
        key = command.get("key", "")
        new_value = command.get("value")

        node = self._find_node_by_path(path)
        if not node:
            return 0

        if prop_type == "attributes":
            node.attributes[key] = new_value
        elif prop_type == "properties":
            node.properties[key] = new_value

        # 重新计算当前节点
        self._recalculate_node_css(node)

        print(f"🔄 更新属性: {node.name}.{key} = {new_value}")
        return 1

    def _handle_delete_value_command(self, command: Dict[str, Any]) -> int:
        """处理属性删除命令"""
        path = command.get("path", [])
        prop_type = command.get("type", "")
        key = command.get("key", "")

        node = self._find_node_by_path(path)
        if not node:
            return 0

        if prop_type == "attributes" and key in node.attributes:
            del node.attributes[key]
        elif prop_type == "properties" and key in node.properties:
            del node.properties[key]

        self._recalculate_node_css(node)

        print(f"🗑️ 删除属性: {node.name}.{key}")
        return 1

    def _recalculate_all_dirty(self) -> int:
        """重新计算所有dirty节点"""
        if not self.dom_root:
            return 0

        count = 0

        def recalc_if_dirty(node: DOMNode):
            nonlocal count
            if node.css_dirty:
                self._recalculate_node_css(node)
                count += 1
            for child in node.children:
                recalc_if_dirty(child)

        recalc_if_dirty(self.dom_root)
        return count

    def _count_nodes(self, node: Optional[DOMNode]) -> int:
        """计算节点总数"""
        if not node:
            return 0

        count = 1
        for child in node.children:
            count += self._count_nodes(child)
        return count

    def get_css_stats(self) -> Dict[str, Any]:
        """获取CSS匹配统计信息"""
        if not self.dom_root:
            return {"total_nodes": 0, "total_rules": len(self.css_rules)}

        def collect_stats(node: DOMNode) -> Dict[str, int]:
            stats = {
                "total_nodes": 1,
                "matched_rules": len(node.matched_rules),
                "computed_styles": len(node.computed_styles),
            }

            for child in node.children:
                child_stats = collect_stats(child)
                stats["total_nodes"] += child_stats["total_nodes"]
                stats["matched_rules"] += child_stats["matched_rules"]
                stats["computed_styles"] += child_stats["computed_styles"]

            return stats

        return collect_stats(self.dom_root)


def parse_css_file(css_file_path: str) -> List[CSSRule]:
    """解析CSS文件并返回CSS规则列表"""
    if not os.path.exists(css_file_path):
        print(f"❌ CSS文件未找到: {css_file_path}")
        return []

    try:
        with open(css_file_path, "r", encoding="utf-8") as f:
            css_content = f.read()

        print(f"📁 读取CSS文件: {css_file_path}")
        rules = parse_css_content(css_content)
        print(f"🎯 解析出 {len(rules)} 条CSS规则")
        return rules

    except Exception as e:
        print(f"❌ 解析CSS文件失败: {e}")
        return []


def parse_css_content(css_content: str) -> List[CSSRule]:
    """解析CSS内容并返回CSS规则列表"""
    rules = []
    priority = 1

    # 移除注释
    css_content = re.sub(r"/\*.*?\*/", "", css_content, flags=re.DOTALL)

    # 处理压缩的CSS - 在}后面添加换行符以便更好地分割规则
    css_content = re.sub(r"}(?!\s*$)", "}\n", css_content)

    # 简单的CSS规则匹配正则表达式
    # 匹配 选择器{属性:值;属性:值;...}
    css_rule_pattern = r"([^{}]+)\{([^{}]*)\}"

    matches = re.findall(css_rule_pattern, css_content)

    for selector_part, properties_part in matches:
        selector_part = selector_part.strip()
        properties_part = properties_part.strip()

        if not selector_part or not properties_part:
            continue

        # 处理多个选择器（逗号分隔）
        selectors = [s.strip() for s in selector_part.split(",") if s.strip()]

        for selector in selectors:
            # 解析属性
            properties = parse_css_properties(properties_part)

            if properties and selector:
                # 过滤掉无效的选择器（比如只包含特殊字符）
                if is_valid_selector(selector):
                    rule = CSSRule(selector, properties, priority)
                    rules.append(rule)
                    priority += 1

    return rules


def parse_css_properties(properties_text: str) -> Dict[str, str]:
    """解析CSS属性字符串，返回属性字典"""
    properties = {}

    # 分割属性，处理分号分隔
    prop_parts = properties_text.split(";")

    for prop_part in prop_parts:
        prop_part = prop_part.strip()
        if not prop_part:
            continue

        # 分割属性名和值
        if ":" in prop_part:
            try:
                prop_name, prop_value = prop_part.split(":", 1)
                prop_name = prop_name.strip()
                prop_value = prop_value.strip()

                # 移除!important等修饰符进行清理
                prop_value = re.sub(r"\s*!important\s*$", "", prop_value)

                if prop_name and prop_value:
                    properties[prop_name] = prop_value
            except ValueError:
                # 处理无效的属性
                continue

    return properties


def is_valid_selector(selector: str) -> bool:
    """检查选择器是否有效"""
    # 过滤掉空选择器或只有空白字符的选择器
    if not selector or not selector.strip():
        return False

    # 过滤掉一些明显无效的选择器模式
    invalid_patterns = [
        r"^\s*$",  # 空白
        r"^[{}]+$",  # 只有大括号
        r"^\s*[,;]+\s*$",  # 只有逗号或分号
    ]

    for pattern in invalid_patterns:
        if re.match(pattern, selector):
            return False

    return True


def load_css_rules() -> List[CSSRule]:
    """动态加载CSS规则（从CSS文件解析）"""
    # 尝试从Google CSS文件加载
    css_file_path = "https___www.google.com_.css"

    rules = parse_css_file(css_file_path)

    # 如果解析失败，提供一些基础的回退规则
    if not rules:
        print("⚠️ CSS文件解析失败，使用基础回退规则")
        rules = [
            CSSRule(
                "body",
                {"margin": "0", "padding": "0", "font-family": "Arial, sans-serif"},
                1,
            ),
            CSSRule("a", {"color": "#1a0dab", "text-decoration": "none"}, 1),
            CSSRule("a:hover", {"text-decoration": "underline"}, 2),
            CSSRule(".button", {"padding": "10px", "border": "1px solid #ccc"}, 1),
        ]

    return rules


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print("Usage: python incremental_css_engine.py <commands_file>")
        sys.exit(1)

    css_rules = load_css_rules()
    print(f"📋 加载了 {len(css_rules)} 条CSS规则")

    engine = IncrementalCSSEngine(css_rules)

    commands_file = sys.argv[1]
    total_commands = 0
    total_affected_nodes = 0

    try:
        with open(commands_file, "r") as f:
            for line_num, line in enumerate(f, 1):
                line = line.strip()
                if not line:
                    continue

                try:
                    command = json.loads(line)
                    affected_count = engine.apply_dom_command(command)

                    total_commands += 1
                    total_affected_nodes += affected_count

                except json.JSONDecodeError as e:
                    print(f"❌ JSON解析错误 (行 {line_num}): {e}")

    except FileNotFoundError:
        print(f"❌ 文件未找到: {commands_file}")
        sys.exit(1)

    stats = engine.get_css_stats()
    print(f"\n📈 处理完成:")
    print(f"  总命令数: {total_commands}")
    print(f"  总影响节点数: {total_affected_nodes}")
    print(f"  DOM节点数: {stats.get('total_nodes', 0)}")
    print(f"  CSS规则数: {len(css_rules)}")
    print(f"  CSS规则匹配数: {stats.get('matched_rules', 0)}")
    print(f"  计算样式数: {stats.get('computed_styles', 0)}")
