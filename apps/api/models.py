from typing import Any, List, Tuple

import dataclasses
import pydantic

Symbol = str
Vector = List["Object"]

class Null(pydantic.BaseModel):
    pass

class Eof(pydantic.BaseModel):
    pass

@dataclasses.dataclass
class Pair():
    car: "Object"
    cdr: "Object"
    
@dataclasses.dataclass
class Number():
    value: int | float | Tuple[int, int]


class Expr(pydantic.BaseModel):
    pass

@dataclasses.dataclass
class Procedure():
    params: List[Symbol]
    body: List[Expr]

@dataclasses.dataclass
class Port(pydantic.BaseModel):
    address: str

@dataclasses.dataclass
class Object():
    value: bool | bytes | str | Eof | Null | Number | Pair | Port | Procedure | Symbol | Vector
