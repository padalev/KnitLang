import init, { parse } from "./pkg/knitlang.js";

const sophiescarf_knitlang = "co6\n{\n{\nk? slyf3\n}7\nk2 kfb k? slyf3\n}22\n{\n{\nk? slyf3\n}7\nk3 skp k? slyf3\n}21\nk2 skp slyf3\n{\nk? slyf3\n}6\nbol3 bor3";
const patch = "co30\n{\nk30\np30\n}20\nbo30";
const aros = "co52\np1 pm p8 pm p34 pm p8 pm p1\n{\nk? m1r sm k8 sm m1l k? m1r sm k8 sm m1l k?\np? m1r sm p8 sm m1l p? m1r sm p8 sm m1l p?\n}8\n{\nk1 m1l k? m1r sm k8 sm m1l k? m1r sm k8 sm m1l k? m1r k1\np? m1r sm p8 sm m1l p? m1r sm p8 sm m1l p?\n}1\n{\nk1 m1l k? sm m1l k? m1r sm k? sm m1l k? m1r sm k? m1r k1\np? sm p? sm p? sm p? sm p?\n}7"//\nk1 m1l k? sm m1l k? m1r sm k? sm m1l k? m1r sm k? m1r co14\n"//cl k?\nsm k? sm k? sm k? sm"
const test = "co9\nk3 pm k6\n{\nk? m1r sm k?\np? sm m1r p?\n}5\nk? rm k?\nbo?";

let pattern;
let row = 0;

async function startApp() {
    try {
        await init();

        //pattern = parse(sophiescarf_knitlang);
        pattern = parse(aros);
        //console.log(pattern.rows[0]);
        //console.log("Success! Parsed result:", pattern);
        //console.log(pattern.rows[0].content.Instructions[0].key + pattern.rows[0].content.Instructions[0].multiplier);
        //return pattern;

    } catch (error) {
        console.error("Failed to initialize or parse WASM:", error);
    }
}
await startApp();

function instruction_string(rowObj) {
    let result = "";
    for (let instruction of rowObj.instructions) {
        let multiplier = instruction.multiplier;
        if (multiplier === undefined) {
            multiplier = "?";
        }
        result += "<div class=\"container\"><div class=\"instruction\">" + instruction.en + "</div><div class=\"multiplier\">" + multiplier + "</div></div>";
    }
    return result
}

function next() {
    row += 1;
    goto();
}

function previous() {
    row -= 1;
    goto();
}


function goto() {
    if (row > pattern.rows.length - 1) {
        row = pattern.rows.length - 1;
    }
    if (row < 0) {
        row = 0;
    }
    let rowObj = pattern.rows[row];
    document.getElementById("current-row-number").innerHTML = row;
    document.getElementById("total-rows").innerHTML = " / " + (pattern.rows.length - 1);
    let stitstring = "";
    for (let stit of rowObj.stitches_out) {
        stitstring += stit + " ";
    }
    document.getElementById("stitches-number").innerHTML = stitstring;
    document.getElementById("instructions").innerHTML = instruction_string(rowObj);
    localStorage.setItem('row', row);
}

function init_js() {
    //console.log("hello world");
    document.getElementById("next").addEventListener("click", next);
    document.getElementById("previous").addEventListener("click", previous);

    let savedRow = localStorage.getItem('row');
    if (savedRow != null) {
        row = Number.parseInt(savedRow);
    }
    goto(row);
}
init_js();
//document.addEventListener("DOMContentLoaded", init_js);
//document.getElementById("next").addEventListener("onload", init_js);

//todo: check against length maybe