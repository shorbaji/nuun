from typing import Any, List, Tuple

import dataclasses
import pydantic

Symbol = str
Vector = List["Object"]

@dataclasses.dataclass
class Null():
    pass

@dataclasses.dataclass
class Eof():
    pass

@dataclasses.dataclass
class Pair():
    car: "Object"
    cdr: "Object"
    
@dataclasses.dataclass
class Number():
    value: int | float | Tuple[int, int]

@dataclasses.dataclass
class Expr():
    pass

@dataclasses.dataclass
class Procedure():
    params: List[Symbol]
    body: List[Expr]

@dataclasses.dataclass
class Port():
    address: str

class Object(pydantic.BaseModel):
    value: bool | bytes | str | Eof | Null | Number | Pair | Port | Procedure | Symbol | Vector

    class Config:
        # do not generate separate schema for Eof, Null, etc
        json_schema_extra = {"by_alias": False}