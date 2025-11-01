from dataclasses import dataclass, field
from typing import Optional

from crcl_python.crclcommands import (
    EndCanonType,
    InitCanonType,
    MiddleCommandType,
)
from crcl_python.data_primitives import DataThingType


@dataclass
class CrclprogramType(DataThingType):
    """CRCLProgramType is derived from DataThingType.

    An instance of CRCLProgramType has the following elements:
    Name (inherited, optional)
    InitCanon
    MiddleCommand (optional, multiple)
    EndCanon.
    CRCLProgramType defines a CRCL program as a sequence of CRCL
    commands. The CRCL commands in an instance of CRCLProgramType
    must be executed in the order given. Using a CRCL program is
    intended for testing and demos, not normal operation.
    A CRCL program must start with an InitCanon command and end
    with an EndCanon command. It may have zero to many middle
    commands between the InitCanon and the EndCanon.
    """

    class Meta:
        name = "CRCLProgramType"

    init_canon: Optional[InitCanonType] = field(
        default=None,
        metadata={
            "name": "InitCanon",
            "type": "Element",
            "required": True,
        },
    )
    middle_command: list[MiddleCommandType] = field(
        default_factory=list,
        metadata={
            "name": "MiddleCommand",
            "type": "Element",
        },
    )
    end_canon: Optional[EndCanonType] = field(
        default=None,
        metadata={
            "name": "EndCanon",
            "type": "Element",
            "required": True,
        },
    )


@dataclass
class Crclprogram(CrclprogramType):
    """
    The global CRCLProgram element may be used as the root element of a CRCL
    instance file containing an entire CRCL program.
    """

    class Meta:
        name = "CRCLProgram"
