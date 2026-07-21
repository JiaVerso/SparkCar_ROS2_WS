import os
import re


def _storage_id_from_metadata(metadata_path):
    """Read rosbag2 storage identifier from metadata.yaml."""
    if not os.path.isfile(metadata_path):
        return None

    with open(metadata_path, "r", encoding="utf-8") as f:
        content = f.read()

    # Example line: "storage_identifier: sqlite3" or "storage_identifier: mcap"
    match = re.search(r"^\s*storage_identifier\s*:\s*([A-Za-z0-9_\-]+)\s*$", content, re.MULTILINE)
    if match:
        return match.group(1)
    return None


def resolve_rosbag2_uri_and_storage_id(bagfile):
    """
    Resolve rosbag2 URI and storage backend from path/metadata.

    Supports:
    - rosbag2 folder path containing metadata.yaml
    - direct .db3/.mcap file path (mapped to parent folder if metadata exists)
    """
    path = os.path.abspath(os.path.expanduser(bagfile))
    uri = path

    if os.path.isfile(path) and path.lower().endswith((".db3", ".mcap")):
        candidate_uri = os.path.dirname(path)
        if os.path.isfile(os.path.join(candidate_uri, "metadata.yaml")):
            uri = candidate_uri

    metadata_path = os.path.join(uri, "metadata.yaml") if os.path.isdir(uri) else os.path.join(os.path.dirname(uri), "metadata.yaml")
    storage_id = _storage_id_from_metadata(metadata_path)
    if storage_id:
        return uri, storage_id

    lower_path = path.lower()
    if lower_path.endswith(".mcap"):
        return uri, "mcap"
    if lower_path.endswith((".db3", ".sqlite3")):
        return uri, "sqlite3"

    # Default rosbag2 backend
    return uri, "sqlite3"
