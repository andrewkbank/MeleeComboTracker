// Fetch the given SVG file and load it into the page.
var svg = document.getElementById("svgobject");
const characters = [
  "Captain Falcon",
	"Donkey Kong",
	'Fox',
	'Mr. Game And Watch',
	'Kirby',
	'Bowser',
	'Link',
	'Luigi',
	'Mario',
	'Marth',
	'Mewtwo',
	'Ness',
	'Peach',
	'Pikachu',
	'Ice Climbers',
	'Jigglypuff',
	'Samus',
	'Yoshi',
	'Zelda',
	'Sheik',
	'Falco',
	'Young Link',
	'Dr. Mario',
	'Roy',
	'Pichu',
	'Ganondorf',
];

svg.addEventListener("load", function(){
  var svgDoc = svg.contentDocument.querySelectorAll('svg');
  var nodes = svgDoc[0].querySelectorAll('path');

  for( var i = 0; i < nodes.length; i++ ) {
    nodes[i].addEventListener('click', changeStroke);
    // Set invisible to start.
    nodes[i].setAttribute('display','none');
  }
  console.log(parsed);
  var lastPathClicked = null;

  const zoomable = document.getElementById("zoomable");
  panzoom = Panzoom(zoomable);
  panzoom.bind();
  if(panzoom)
    console.log("Panzoom successfully initialized.");
  document.addEventListener('wheel', panzoom.zoomWithWheel);
  document.addEventListener(
  "keydown",
  (event) => {
    const keyName = event.key;
    console.log(keyName);
    if(keyName === 'ArrowLeft')
      panzoom.pan(10, 0, {relative:true});
    if(keyName === 'ArrowRight')
      panzoom.pan(-10, 0, {relative:true});
    if(keyName === 'ArrowUp')
      panzoom.pan(0, 10, {relative:true});
    if(keyName === 'ArrowDown')
      panzoom.pan(0, -10, {relative:true});
  });

  //Add checkboxes to filter.
  const filterContainer = document.getElementById('filter-container');
  var id = 0;
  characters.forEach(element => {
    var checkbox = document.createElement('input');
    checkbox.type = 'checkbox';
    checkbox.name = id;
    checkbox.id = id;
    checkbox.value = 'no';

    var label = document.createElement('label');
    label.htmlFor = id;
    label.appendChild(document.createTextNode(element)); 
    label.appendChild(document.createElement('br')); 

    filterContainer.appendChild(checkbox);
    filterContainer.appendChild(label);
    checkbox.addEventListener('change', function(){
  console.log(this.checked);
    if(!parsed)
      return;
    // Go through all the paths and toggle based on this checkbox's setting.
    nodes.forEach(path => {
      if(parsed[path.id][6] === this.id)
        path.setAttribute('display', (this.checked ? 'inline':'none'));
    });
    });
    id += 1;
  });

  const filterCollapsable = document.getElementById('filter-collapsable');
  filterCollapsable.addEventListener('click', function(){
    this.classList.toggle('active');
    var content = this.nextElementSibling;
    if(content.style.display === "block")
      content.style.display = 'none';
    else
      content.style.display = 'block';
    });

  function changeStroke() {
    this.setAttribute('opacity', 0.90);
    this.setAttribute('stroke', 'red');
    if(lastPathClicked)
    {
      lastPathClicked.setAttribute('opacity', 0.10);
      lastPathClicked.setAttribute('stroke', 'black');
    }
    lastPathClicked = this;

    const id = this.getAttribute("id");
    const start_percent = parsed[id][8];
    const end_percent = parsed[id][9];
    const text = 
      'Attacker: ' + characters[parsed[id][6]] + '\n' +
      'Defender: ' + characters[parsed[id][7]] + '\n' +
      "Start percent: " + start_percent + "\n" +
      "End percent: " + end_percent + "\n";

    document.getElementById("info").innerText = text;
  }
})
