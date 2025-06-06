# path is a list of int.
from typing import List, Dict, Any, Optional, Union


def command_init(node: Dict[str, Any], time: Union[int, float]) -> Dict[str, Any]:
    return {"name": "init", "node": node, "time": time}


def command_layout_init(node: Dict[str, Any]) -> Dict[str, Any]:
    return {"name": "layout_init", "node": node}


def command_recalculate(time: Union[int, float]) -> Dict[str, Any]:
    return {"name": "recalculate", "time": time}


def command_add(path: List[int], node: Dict[str, Any]) -> Dict[str, Any]:
    return {"name": "add", "path": path, "node": node}


def command_remove(path: List[int], old_node: Dict[str, Any]) -> Dict[str, Any]:
    return {"name": "remove", "path": path, "old_node": old_node}


def command_layout_add(path: List[int], node: Dict[str, Any]) -> Dict[str, Any]:
    return {"name": "layout_add", "path": path, "node": node}


def command_layout_remove(path: List[int], old_node: Dict[str, Any]) -> Dict[str, Any]:
    return {"name": "layout_remove", "path": path, "old_node": old_node}


def command_replace(
    path: List[int], old_node: Dict[str, Any], node: Dict[str, Any]
) -> Dict[str, Any]:
    return {"name": "replace", "path": path, "old_node": old_node, "node": node}


def command_layout_replace(
    path: List[int], old_node: Dict[str, Any], node: Dict[str, Any]
) -> Dict[str, Any]:
    return {"name": "layout_replace", "path": path, "old_node": old_node, "node": node}


def command_replace_value(
    path: List[int], on: str, type_: str, k: str, old_value: Any, v: Any
) -> Dict[str, Any]:
    return {
        "name": "replace_value",
        "path": path,
        "on": on,
        "type": type_,
        "key": k,
        "old_value": old_value,
        "value": v,
    }


def command_insert_value(
    path: List[int], on: str, type_: str, k: str, v: Any
) -> Dict[str, Any]:
    return {
        "name": "insert_value",
        "path": path,
        "on": on,
        "type": type_,
        "key": k,
        "value": v,
    }


def command_delete_value(
    path: List[int], on: str, type_: str, k: str, old_value: Any
) -> Dict[str, Any]:
    return {
        "name": "delete_value",
        "path": path,
        "on": on,
        "type": type_,
        "key": k,
        "old_value": old_value,
    }


def command_layout_info_changed(
    path: List[int], old: Dict[str, Any], new: Dict[str, Any]
) -> Dict[str, Any]:
    return {"name": "layout_info_changed", "path": path, "old": old, "new": new}


def size(j: Dict[str, Any]) -> int:
    ret: int = 1
    if "children" not in j:
        return ret
    for c in j["children"]:
        ret += size(c)
    return ret


def regularize_dom(j: Dict[str, Any]) -> Optional[Dict[str, Any]]:
    if "id" not in j:
        return None
    if "name" not in j:
        return None
    elif j["name"] == "#comment":
        return None
    elif j["name"] == "#doctype":
        return None
    else:
        if "children" not in j:
            j["children"] = []
        tmp: List[Dict[str, Any]] = []
        for c in j["children"]:
            c = regularize_dom(c)
            if c is not None:
                tmp.append(c)
        j["children"] = tmp

        if "attributes" not in j:
            j["attributes"] = {}

        if "properties" not in j:
            j["properties"] = {}

        return j


def regularize_layout(j: Dict[str, Any]) -> Dict[str, Any]:
    if "children" not in j:
        j["children"] = []
    tmp: List[Dict[str, Any]] = []
    for c in j["children"]:
        c = regularize_layout(c)
        tmp.append(c)
    j["children"] = tmp

    if "x" not in j:
        j["x"] = 0
    if "y" not in j:
        j["y"] = 0
    if "width" not in j:
        j["width"] = 0
    if "height" not in j:
        j["height"] = 0
    return j


trace_list: List[str] = [
    "aliyun",
    "amazon",
    "anydesk",
    "apple",
    "archive",
    "arxiv",
    "bananaspace",
    "bilibili",
    "booking",
    "chess",
    "coursera",
    "cppreference",
    "discord_nologin",
    "espn",
    "expedia",
    "firefox",
    "github_commits",
    "github_nologin",
    "github_repo",
]
trace_list.append("gmail_nologin")
trace_list.append("google_hover")
trace_list.append("google_play")
trace_list.append("google_scholar")
trace_list.append("google_searchbar")
trace_list.append("google_searchpage")
trace_list.append("googlesource_chromium")
trace_list.append("haskell")
trace_list.append("hn_type")
trace_list.append("hsr")
trace_list.append("hsreplay")
trace_list.append("janestreet_main")
trace_list.append("jetbrains")
trace_list.append("lichess_anal")
trace_list.append("mihoyo")
trace_list.append("ocaml")
trace_list.append("panda_express")
trace_list.append("reddit")
trace_list.append("spotify")
trace_list.append("stack_overflow_main")
trace_list.append("steam")
trace_list.append("twitter_login")
trace_list.append("twitter_main")
trace_list.append("vscode")
trace_list.append("walmart")
trace_list.append("w3school")
trace_list.append("wikipedia_hover")
trace_list.append("wikipedia_idle")
trace_list.append("windows")
trace_list.append("yahoo")
trace_list.append("youtube")

# trace_list=["wikipedia_hover", "twitter_main"]
