from fafreplay import commands


def test_command_names():
    assert commands.NAMES[commands.Advance] == "Advance"
    assert commands.NAMES[commands.SetCommandSource] == "SetCommandSource"
    assert commands.NAMES[commands.CommandSourceTerminated] == "CommandSourceTerminated"
    assert commands.NAMES[commands.VerifyChecksum] == "VerifyChecksum"
    assert commands.NAMES[commands.RequestPause] == "RequestPause"
    assert commands.NAMES[commands.Resume] == "Resume"
    assert commands.NAMES[commands.SingleStep] == "SingleStep"
    assert commands.NAMES[commands.CreateUnit] == "CreateUnit"
    assert commands.NAMES[commands.CreateProp] == "CreateProp"
    assert commands.NAMES[commands.DestroyEntity] == "DestroyEntity"
    assert commands.NAMES[commands.WarpEntity] == "WarpEntity"
    assert commands.NAMES[commands.ProcessInfoPair] == "ProcessInfoPair"
    assert commands.NAMES[commands.IssueCommand] == "IssueCommand"
    assert commands.NAMES[commands.IssueFactoryCommand] == "IssueFactoryCommand"
    assert commands.NAMES[commands.IncreaseCommandCount] == "IncreaseCommandCount"
    assert commands.NAMES[commands.DecreaseCommandCount] == "DecreaseCommandCount"
    assert commands.NAMES[commands.SetCommandTarget] == "SetCommandTarget"
    assert commands.NAMES[commands.SetCommandType] == "SetCommandType"
    assert commands.NAMES[commands.SetCommandCells] == "SetCommandCells"
    assert commands.NAMES[commands.RemoveCommandFromQueue] == "RemoveCommandFromQueue"
    assert commands.NAMES[commands.DebugCommand] == "DebugCommand"
    assert commands.NAMES[commands.ExecuteLuaInSim] == "ExecuteLuaInSim"
    assert commands.NAMES[commands.LuaSimCallback] == "LuaSimCallback"
    assert commands.NAMES[commands.EndGame] == "EndGame"

    assert len(commands.NAMES) == commands.MAX + 1
