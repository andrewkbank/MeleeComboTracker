// Pass CSV information containing combo information to a CSV reader
import {parse} from 'https://cdn.jsdelivr.net/npm/@vanillaes/csv@3.0.1/+esm';
// TODO: Provide error message on import fail.

// Fetch csv blob from local files
const response = await fetch("combos2.csv")
  .then((combos) => combos.text())
  .then((text) => {parsed = parse(text);})
  .catch((exception) => console.error(exception));

// Parse it into a 2d array for the rest of the program
console.log(parsed ? "Parse successful." : "Parse failed.");
