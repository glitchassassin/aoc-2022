const fs = require('fs');

/**
 * 
 * @param {string} input 
 */
const parseValves = (input) => {
    const valves = {};
    for (const line of input.split('\n')) {
        const matches = /Valve (?<name>\w+) has flow rate=(?<flowRate>\d+); tunnels? leads? to valves? (?<tunnels>.+)/.exec(line);
        if (matches) {
            const { name, flowRate, tunnels } = matches.groups;
            valves[name] = {
                flowRate: parseInt(flowRate),
                tunnels: tunnels.split(', ')
            }
        }
    }
    return valves
}

const simplifyValves = (valves) => {
    for (const v in valves) {
        if (valves[v].flowRate === 0) continue;
        
    }
}

/**
 * Calculate distance to every other valve with a flow rate greater than zero
 */
const valveDistances = (valves, start) => {
    const frontier = [start];
    const visited = [];
    while (frontier.length) {
        const v = frontier.shift();
    }
}


console.log(parseValves(fs.readFileSync('inputs/sample.txt', 'utf-8')));