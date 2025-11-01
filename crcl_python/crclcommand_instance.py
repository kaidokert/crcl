from dataclasses import dataclass, field
from typing import Optional

from crcl_python.crclcommands import CrclcommandType
from crcl_python.data_primitives import DataThingType


@dataclass
class CrclcommandInstanceType(DataThingType):
    """CRCLCommandInstanceType is derived from DataThingType.

    An instance of CRCLCommandInstanceType has the following elements:
    Name (inherited, optional)
    CRCLCommand
    ProgramFile (optional)
    ProgramIndex (optional)
    ProgramLength (optional).
    ProgramFile provides an optional reference if the currently executing command
    is known to have come from a particular file.
    ProgramIndex provoides an optional reference to the element within a program. If the
    currently executing command is known to have come from a particular file. The InitCanon command will have
    index 0, and first MiddleCommand will have index 1.
    ProgramLength is the number of commands in the current program if known.
    CRCLCommandInstanceType contains a single CRCL command.
    """

    class Meta:
        name = "CRCLCommandInstanceType"

    crclcommand: Optional[CrclcommandType] = field(
        default=None,
        metadata={
            "name": "CRCLCommand",
            "type": "Element",
            "required": True,
        },
    )
    program_file: Optional[str] = field(
        default=None,
        metadata={
            "name": "ProgramFile",
            "type": "Element",
        },
    )
    program_index: Optional[int] = field(
        default=None,
        metadata={
            "name": "ProgramIndex",
            "type": "Element",
        },
    )
    program_length: Optional[int] = field(
        default=None,
        metadata={
            "name": "ProgramLength",
            "type": "Element",
        },
    )


@dataclass
class CrclcommandInstance(CrclcommandInstanceType):
    """
    The global CRCLCommandInstance element may be used as the root element of a
    CRCL instance file containing a single CRCL command.
    """

    class Meta:
        name = "CRCLCommandInstance"
