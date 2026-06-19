const sophiescarf_knitlang = "co6\n{\n{\nk? slyf3\n}7\nk2 kfb k? slyf3\n}22\n{\n{\nk? slyf3\n}7\nk3 skp k? slyf3\n}21\nk2 skp slyf3\n{\nk? slyf3\n}6\nbol3 bor3";

let row = 0;

/**
 * Processes the file content string and returns a flattened array of lines.
 * @param {string} content - The raw string content of the file.
 * @returns {string[]} An array of expanded string rows.
 * @throws {Error} If the structure has mismatched or unclosed brackets.
 */
function processContent(content) {
    // Stack of arrays to manage nesting levels
    const stack = [[]];

    // Split content by lines (handles both \n and \r\n)
    const lines = content.split(/\r?\n/);

    for (let line of lines) {
        line = line.trim();

        // Skip empty lines
        if (line === "") {
            continue;
        }

        if (line === "{") {
            // Push a new layer onto the stack for the upcoming sub-loop
            stack.push([]);
        } else if (line.startsWith("}")) {
            // Error check: hit a '}' but we aren't inside a loop
            if (stack.length < 2) {
                throw new Error(
                    `Malformed file: Found closing bracket '${line}' without a matching '{'`
                );
            }

            // Pop the current loop's collected rows
            const currentLoopRows = stack.pop();

            // Extract and parse the multiplier (e.g., "}3" -> 3)
            const multiplierStr = line.slice(1);
            let repeatCount = parseInt(multiplierStr, 10);

            if (isNaN(repeatCount)) {
                console.warn(`Warning: Invalid loop count '${multiplierStr}', defaulting to 1`);
                repeatCount = 1;
            }

            // Expand and append these rows to the parent level below it
            const parentLevel = stack[stack.length - 1];
            for (let i = 0; i < repeatCount; i++) {
                parentLevel.push(...currentLoopRows);
            }
        } else {
            // It's a regular row, push it to the current top level of the stack
            const currentLevel = stack[stack.length - 1];
            currentLevel.push(line);
        }
    }

    // Error check: if stack size > 1, a '{' was never closed
    if (stack.length !== 1) {
        throw new Error(
            "Malformed file: One or more opening brackets '{' were never closed"
        );
    }

    // Return the completed, flattened array
    return stack.pop();
}
const rows = processContent(sophiescarf_knitlang);

/**
 * Processes rows of stitch commands and tracks the stitch count.
 * @param {string[]} rows - An array of row strings.
 * @returns {number[]} An array of stitch counts per row.
 */
function processStitches(rowsi) {
    const stitches = [];
    let stitchCount = 0;

    for (const rowi of rowsi) {
        // Split by any consecutive whitespace characters
        const tokens = rowi.trim().split(/\s+/);

        for (const stitch of tokens) {
            // Skip empty tokens (e.g. if there were leading/trailing spaces)
            if (!stitch) continue;

            // Find the index of the first ASCII digit
            const firstDigitIdx = stitch.search(/\d/);

            let command, multiplierStr;
            if (firstDigitIdx !== -1) {
                command = stitch.slice(0, firstDigitIdx);
                multiplierStr = stitch.slice(firstDigitIdx);
            } else {
                command = stitch;
                multiplierStr = "";
            }

            // Parse the multiplier, defaulting to 1 if none or invalid
            let multiplier = parseInt(multiplierStr, 10);
            if (isNaN(multiplier)) {
                multiplier = 1;
            }

            // Match the command and update the counter
            switch (command) {
                case "co":
                case "kfb":
                    stitchCount += multiplier;
                    break;
                case "skp":
                case "bol":
                case "bor":
                    stitchCount -= multiplier;
                    break;
                default:
                    // Equivalent to Rust' _ => {}
                    break;
            }
        }

        stitches.push(stitchCount);
    }

    return stitches;
}
const stitches = processStitches(rows);

function update() {
    document.getElementById("current-row-number").innerHTML = row;
    document.getElementById("total-rows").innerHTML = " / " + rows.length;
    document.getElementById("stitches-number").innerHTML = stitches[row];
    document.getElementById("instructions").innerHTML = rows[row];
}

function next() {
    row = Math.min((row + 1), rows.length);
    update();
}

function previous() {
    row = Math.max((row - 1), 0);
    update();
}

function init() {
    document.getElementById("next").addEventListener("click", next);
    document.getElementById("previous").addEventListener("click", previous);

    update();
}
document.addEventListener("DOMContentLoaded", init);