import init from "./scripts/bspts.js";
import {greet} from "./scripts/bspts.js";

async function startup() {
    await init();
    greet();
}

startup();