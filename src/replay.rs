use crate::lua::{table_into_py, LuaObject};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use faf_replay_parser::scfa::replay::GameCommand;

pub struct Replay(pub faf_replay_parser::scfa::replay::Replay);
pub struct ReplayHeader(pub faf_replay_parser::scfa::replay::ReplayHeader);
pub struct ReplayBody(pub faf_replay_parser::scfa::replay::ReplayBody);
pub struct SimData(pub faf_replay_parser::scfa::replay::SimData);
pub struct ReplayCommand(pub faf_replay_parser::scfa::replay::ReplayCommand);
pub struct Position(pub faf_replay_parser::scfa::replay::Position);
pub struct Target(pub faf_replay_parser::scfa::replay::Target);
pub struct Formation(pub faf_replay_parser::scfa::replay::Formation);

impl IntoPy<PyObject> for Replay {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item::<&str, PyObject>("header", ReplayHeader(self.0.header).into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("body", ReplayBody(self.0.body).into_py(py))
            .unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for ReplayHeader {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item::<&str, String>("scfa_version", self.0.scfa_version)
            .unwrap();
        res.set_item("replay_version", self.0.replay_version).unwrap();
        res.set_item("map_file", self.0.map_file).unwrap();
        res.set_item::<&str, PyObject>("mods", LuaObject(self.0.mods).into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("scenario", LuaObject(self.0.scenario).into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("players", self.0.players.into_py(py))
            .unwrap();
        res.set_item("cheats_enabled", self.0.cheats_enabled).unwrap();
        res.set_item("army_count", self.0.army_count).unwrap();
        res.set_item::<&str, PyObject>("armies", table_into_py(self.0.armies, py))
            .unwrap();
        res.set_item("seed", self.0.seed).unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for ReplayBody {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item::<&str, PyObject>("sim", SimData(self.0.sim).into_py(py))
            .unwrap();
        res.set_item::<&str, PyObject>("commands", self.0.commands.into_iter().map(|c| ReplayCommand(c)).collect::<Vec<ReplayCommand>>().into_py(py))
            .unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for SimData {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item("tick", self.0.tick).unwrap();
        res.set_item("command_source", self.0.command_source).unwrap();
        res.set_item("players_last_tick", self.0.players_last_tick)
            .unwrap();
        // Bytes are copied
        res.set_item("checksum", PyBytes::new(py, &self.0.checksum))
            .unwrap();
        res.set_item("checksum_tick", self.0.checksum_tick).unwrap();
        res.set_item("desync_tick", self.0.desync_tick).unwrap();
        res.set_item("desync_ticks", self.0.desync_ticks).unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for ReplayCommand {
    fn into_py(self, py: Python) -> PyObject {
        use faf_replay_parser::scfa::replay::ReplayCommand::*;
        let res = PyDict::new(py);

        match self.0 {
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
                res.set_item::<&str, PyObject>("position", Position(position).into_py(py))
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
                set_game_command_items(res, game_command, py);
            }
            IssueFactoryCommand(game_command) => {
                res.set_item("name", "IssueFactoryCommand").unwrap();
                set_game_command_items(res, game_command, py);
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
                res.set_item::<&str, PyObject>("target", Target(target).into_py(py))
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
                res.set_item::<&str, PyObject>("cells", LuaObject(cells).into_py(py))
                    .unwrap();
                res.set_item::<&str, PyObject>("position", Position(position).into_py(py))
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
                res.set_item::<&str, PyObject>("position", Position(position).into_py(py))
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
                res.set_item::<&str, PyObject>("args", LuaObject(args).into_py(py))
                    .unwrap();
                res.set_item("selection", selection).unwrap();
            }
            EndGame => res.set_item("name", "EndGame").unwrap(),
        }

        res.into_py(py)
    }
}

fn set_game_command_items(res: &PyDict, game_command: GameCommand, py: Python) {
    res.set_item("entity_ids", game_command.entity_ids).unwrap();
    res.set_item("id", game_command.id).unwrap();
    res.set_item("coordinated_attack_cmd_id", game_command.coordinated_attack_cmd_id).unwrap();
    res.set_item("type", game_command.type_).unwrap();
    res.set_item("arg2", game_command.arg2).unwrap();
    res.set_item::<&str, PyObject>("target", Target(game_command.target).into_py(py)).unwrap();
    res.set_item("arg3", game_command.arg3).unwrap();
    res.set_item::<&str, PyObject>("formation", game_command.formation.map(|f| Formation(f)).into_py(py)).unwrap();
    res.set_item("blueprint", game_command.blueprint).unwrap();
    res.set_item("arg4", game_command.arg4).unwrap();
    res.set_item("arg5", game_command.arg5).unwrap();
    res.set_item("arg6", game_command.arg6).unwrap();
    res.set_item::<&str, PyObject>("upgrades", LuaObject(game_command.upgrades).into_py(py)).unwrap();
    res.set_item("clear_queue", game_command.clear_queue).unwrap();
}

impl IntoPy<PyObject> for Position {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item("x", self.0.x).unwrap();
        res.set_item("y", self.0.y).unwrap();
        res.set_item("z", self.0.z).unwrap();

        res.into_py(py)
    }
}

impl IntoPy<PyObject> for Target {
    fn into_py(self, py: Python) -> PyObject {
        use faf_replay_parser::scfa::replay::Target::*;

        match self.0 {
            None => py.None(),
            Entity { id } => {
                let res = PyDict::new(py);
                res.set_item("id", id).unwrap();
                res.into_py(py)
            }
            Position(p) => self::Position(p).into_py(py),
        }
    }
}

impl IntoPy<PyObject> for Formation {
    fn into_py(self, py: Python) -> PyObject {
        let res = PyDict::new(py);

        res.set_item("a", self.0.a).unwrap();
        res.set_item("b", self.0.b).unwrap();
        res.set_item("c", self.0.c).unwrap();
        res.set_item("d", self.0.d).unwrap();
        res.set_item("scale", self.0.scale).unwrap();

        res.into_py(py)
    }
}
