use faf_replay_parser::scfa::replay::*;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

impl IntoPy<PyObject> for Replay {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item::<&str, PyObject>("header", self.header.into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("body", self.body.into_py(py))
            .unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for ReplayHeader {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item::<&str, String>("scfa_version", self.scfa_version)
            .unwrap();
        res.set_item("replay_version", self.replay_version).unwrap();
        res.set_item("map_file", self.map_file).unwrap();
        res.set_item::<&str, PyObject>("mods", self.mods.into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("scenario", self.scenario.into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("players", self.players.into_py(py))
            .unwrap();
        res.set_item("cheats_enabled", self.cheats_enabled).unwrap();
        res.set_item("army_count", self.army_count).unwrap();
        res.set_item::<&str, PyObject>("armies", self.armies.into_py(py))
            .unwrap();
        res.set_item("seed", self.seed).unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for ReplayBody {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item::<&str, PyObject>("sim", self.sim.into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("commands", self.commands.into_py(py))
            .unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for SimData {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item("tick", self.tick).unwrap();
        res.set_item("command_source", self.command_source).unwrap();
        res.set_item("players_last_tick", self.players_last_tick)
            .unwrap();
        // Bytes are copied
        res.set_item("checksum", PyBytes::new(py, &self.checksum))
            .unwrap();
        res.set_item("checksum_tick", self.checksum_tick).unwrap();
        res.set_item("desync_tick", self.desync_tick).unwrap();
        res.set_item("desync_ticks", self.desync_ticks).unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for ReplayCommand {
    fn into_py(self, py: Python) -> PyObject {
        use ReplayCommand::*;
        let res = PyDict::new(py);

        match self {
            Advance { ticks } => {
                res.set_item("name", "Advance").unwrap();
                res.set_item("ticks", ticks).unwrap();
            }
            SetCommandSource { id } => {
                res.set_item("name", "SetCommandSource").unwrap();
                res.set_item("id", id).unwrap();
            }
            CommandSourceTerminated => res.set_item("name", "CommandSourceTerminated").unwrap(),
            VerifyChecksum { digest, tick } => {
                res.set_item("name", "VerifyChecksum").unwrap();
                res.set_item("digest", PyBytes::new(py, &digest)).unwrap();
                res.set_item("tick", tick).unwrap();
            }
            RequestPause => res.set_item("name", "RequestPause").unwrap(),
            Resume => res.set_item("name", "Resume").unwrap(),
            SingleStep => res.set_item("name", "SingleStep").unwrap(),
            CreateUnit { army, blueprint, x, z, heading } => {
                res.set_item("name", "CreateUnit").unwrap();
                res.set_item("army", army).unwrap();
                res.set_item("blueprint", blueprint).unwrap();
                res.set_item("x", x).unwrap();
                res.set_item("z", z).unwrap();
                res.set_item("heading", heading).unwrap();
            }
            CreateProp { blueprint, position } => {
                res.set_item("name", "CreateProp").unwrap();
                res.set_item("blueprint", blueprint).unwrap();
                res.set_item::<&str, PyObject>("position", position.into_py(py))
                    .unwrap();
            }
            DestroyEntity { unit } => {
                res.set_item("name", "DestroyEntity").unwrap();
                res.set_item("unit", unit).unwrap();
            }
            WarpEntity { unit, x, y, z } => {
                res.set_item("name", "WarpEntity").unwrap();
                res.set_item("unit", unit).unwrap();
                res.set_item("x", x).unwrap();
                res.set_item("y", y).unwrap();
                res.set_item("z", z).unwrap();
            }
            ProcessInfoPair { unit, arg1, arg2 } => {
                res.set_item("name", "ProcessInfoPair").unwrap();
                res.set_item("unit", unit).unwrap();
                res.set_item("arg1", arg1).unwrap();
                res.set_item("arg2", arg2).unwrap();
            }
            IssueCommand(game_command) => {
                res.set_item("name", "IssueCommand").unwrap();
                // TODO: Copy all game command items to res
            }
            IssueFactoryCommand(game_command) => {
                res.set_item("name", "IssueFactoryCommand").unwrap();
                // TODO: Copy all game command items to res
            }
            IncreaseCommandCount { id, delta } => {
                res.set_item("name", "IncreaseCommandCount").unwrap();
                res.set_item("id", id).unwrap();
                res.set_item("delta", delta).unwrap();
            }
            DecreaseCommandCount { id, delta } => {
                res.set_item("name", "DecreaseCommandCount").unwrap();
                res.set_item("id", id).unwrap();
                res.set_item("delta", delta).unwrap();
            }
            SetCommandTarget { id, target } => {
                res.set_item("name", "SetCommandTarget").unwrap();
                res.set_item("id", id).unwrap();
                res.set_item::<&str, PyObject>("target", target.into_py(py))
                    .unwrap();
            }
            SetCommandType { id, type_ } => {
                res.set_item("name", "SetCommandType").unwrap();
                res.set_item("id", id).unwrap();
                res.set_item("type_", type_).unwrap();
            }
            SetCommandCells { id, cells, position } => {
                res.set_item("name", "SetCommandCells").unwrap();
                res.set_item("id", id).unwrap();
                res.set_item::<&str, PyObject>("cells", cells.into_py(py))
                    .unwrap();
                res.set_item::<&str, PyObject>("position", position.into_py(py))
                    .unwrap();
            }
            RemoveCommandFromQueue { id, unit } => {
                res.set_item("name", "RemoveCommandFromQueue").unwrap();
                res.set_item("id", id).unwrap();
                res.set_item("unit", unit).unwrap();
            }
            DebugCommand { command, position, focus_army, selection } => {
                res.set_item("name", "DebugCommand").unwrap();
                res.set_item("command", command).unwrap();
                res.set_item::<&str, PyObject>("position", position.into_py(py))
                    .unwrap();
                res.set_item("focus_army", focus_army).unwrap();
                res.set_item("selection", selection).unwrap();
            }
            ExecuteLuaInSim { code } => {
                res.set_item("name", "ExecuteLuaInSim").unwrap();
                res.set_item("code", code).unwrap();
            }
            LuaSimCallback { func, args, selection } => {
                res.set_item("name", "LuaSimCallback").unwrap();
                res.set_item("func", func).unwrap();
                res.set_item::<&str, PyObject>("args", args.into_py(py))
                    .unwrap();
                res.set_item("selection", selection).unwrap();
            }
            EndGame => res.set_item("name", "EndGame").unwrap(),
        }

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for Position {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item("x", self.x).unwrap();
        res.set_item("y", self.y).unwrap();
        res.set_item("z", self.z).unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for Target {
    fn into_py(self, py: Python) -> PyObject {
        use Target::*;

        match self {
            None => py.None(),
            Entity { id } => {
                let res = PyDict::new(py);
                res.set_item("id", id).unwrap();
                res.into_py(py)
            }
            Position(p) => p.into_py(py),
        }
    }
}
