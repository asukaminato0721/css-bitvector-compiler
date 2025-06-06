#!/usr/bin/env python3
"""
调试脚本：显示解析出的CSS规则详情
"""

from incremental_css_engine import parse_css_file


def main():
    css_file_path = "https___www.google.com_.css"
    rules = parse_css_file(css_file_path)

    print(f"\n🔍 显示前20条解析出的CSS规则：\n")

    for i, rule in enumerate(rules[:20]):
        print(f"规则 {i + 1}:")
        print(f"  选择器: {rule.selector}")
        print(f"  优先级: {rule.priority}")
        print(f"  属性数量: {len(rule.properties)}")
        print(
            f"  属性: {dict(list(rule.properties.items())[:3])}..."
        )  # 只显示前3个属性
        print()

    print(f"\n📊 统计信息：")
    print(f"  总规则数: {len(rules)}")

    # 统计选择器类型
    id_selectors = sum(1 for rule in rules if rule.selector.startswith("#"))
    class_selectors = sum(1 for rule in rules if rule.selector.startswith("."))
    tag_selectors = sum(1 for rule in rules if not rule.selector.startswith(("#", ".")))
    pseudo_selectors = sum(1 for rule in rules if ":" in rule.selector)

    print(f"  ID选择器: {id_selectors}")
    print(f"  类选择器: {class_selectors}")
    print(f"  标签选择器: {tag_selectors}")
    print(f"  伪类选择器: {pseudo_selectors}")

    # 统计最常用的属性
    all_properties = {}
    for rule in rules:
        for prop in rule.properties.keys():
            all_properties[prop] = all_properties.get(prop, 0) + 1

    top_properties = sorted(all_properties.items(), key=lambda x: x[1], reverse=True)[
        :10
    ]
    print(f"\n🏆 最常用的10个CSS属性:")
    for prop, count in top_properties:
        print(f"  {prop}: {count}次")


if __name__ == "__main__":
    main()
