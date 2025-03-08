import dataclasses

@dataclasses.dataclass
class Error(Exception):
    message: str
    code: int
    data: dict[str, str] | None
