// Fetch the given SVG file and load it into the page.
var svg = document.getElementById("svgobject");
const characters = [
  "Mario",
  "Fox",
  "CaptainFalcon",
  "DonkeyKong",
  "Kirby",
  "Bowser",
  "Link",
  "Sheik",
  "Ness",
  "Peach",
  "IceClimbers",
  "Nana",
  "Pikachu",
  "Samus",
  "Yoshi",
  "Jigglypuff",
  "Mewtwo",
  "Luigi",
  "Marth",
  "Zelda",
  "YoungLink",
  "DrMario",
  "Falco",
  "Pichu",
  "GameAndWatch",
  "Ganondorf",
  "Roy",
  "MasterHand",
  "CrazyHand",
  "WireFrameMale",
  "WireFrameFemale",
  "GigaBowser",
  "Sandbag"
];

const attack_ids = [
  "Grab",
  "Taunt/Item Throw",
  "Jab 1",
  "Jab 2",
  "Jab 3",
  "Rapid Jabs",
  "Dash Attack",
  "Side Tilt",
  "Up Tilt",
  "Down Tilt",
  "Side Smash",
  "Up Smash",
  "Down Smash",
  "Nair",
  "Fair",
  "Bair",
  "Uair",
  "Dair",
  "Neutral Special",
  "Side Special",
  "Up Special",
  "Down Special",
  "Kirby Hat: Mario Neutral Special",
  "Kirby Hat: Fox Neutral Special",
  "Kirby Hat: CFalcon Neutral Special",
  "Kirby Hat: DK Neutral Special",
  "Kirby Hat: Bowser Neutral Special",
  "Kirby Hat: Link Neutral Special",
  "Kirby Hat: Sheik Neutral Special",
  "Kirby Hat: Ness Neutral Special",
  "Kirby Hat: Peach Neutral Special",
  "Kirby Hat: Ice Climber Neutral Special",
  "Kirby Hat: Pikachu Neutral Special",
  "Kirby Hat: Samus Neutral Special",
  "Kirby Hat: Yoshi Neutral Special",
  "Kirby Hat: Jigglypuff Neutral Special",
  "Kirby Hat: Mewtwo Neutral Special",
  "Kirby Hat: Luigi Neutral Special",
  "Kirby Hat: Marth Neutral Special",
  "Kirby Hat: Zelda Neutral Special",
  "Kirby Hat: Young Link Neutral Special",
  "Kirby Hat: Doc Neutral Special",
  "Kirby Hat: Falco Neutral Special",
  "Kirby Hat: Pichu Neutral Special",
  "Kirby Hat: Game & Watch Neutral Special",
  "Kirby Hat: Ganon Neutral Special",
  "Kirby Hat: Roy Neutral Special",
  "Unk",
  "Unk",
  "Unk",
  "Get Up Attack (From Back)",
  "Get Up Attack (From Front)",
  "Pummel",
  "Forward Throw",
  "Back Throw",
  "Up Throw",
  "Down Throw",
  "Cargo Forward Throw",
  "Cargo Back Throw",
  "Cargo Up Throw",
  "Cargo Down Throw",
  "Ledge Get Up Attack 100%+",
  "Ledge Get Up Attack",
  "Beam Sword Jab",
  "Beam Sword Tilt Swing",
  "Beam Sword Smash Swing",
  "Beam Sword Dash Swing",
  "Home Run Bat Jab",
  "Home Run Bat Tilt Swing",
  "Home Run Bat Smash Swing",
  "Home Run Bat Dash Swing",
  "Parasol Jab",
  "Parasol Tilt Swing",
  "Parasol Smash Swing",
  "Parasol Dash Swing",
  "Fan Jab",
  "Fan Tilt Swing",
  "Fan Smash Swing",
  "Fan Dash Swing",
  "Star Rod Jab",
  "Star Rod Tilt Swing",
  "Star Rod Smash Swing",
  "Star Rod Dash Swing",
  "Lip's Stick Jab",
  "Lip's Stick Tilt Swing",
  "Lip's Stick Smash Swing",
  "Lip's Stick Dash Swing",
  "Open Parasol",
  "Ray Gun Shoot",
  "Fire Flower Shoot",
  "Screw Attack",
  "Super Scope (Rapid)",
  "Super Scope (Charged)",
  "Hammer"
]

svg.addEventListener("load", function(){
  console.log(svg);
  var svgDoc = svg.contentDocument.querySelectorAll('svg');
  var nodes = svgDoc[0].querySelectorAll('path');

  for( var i = 0; i < nodes.length; i++ ) {
    nodes[i].addEventListener('click', changeStroke);
    // Set invisible to start.
    nodes[i].setAttribute('display','none');
  }
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
    console.log(id);
    console.log(parsed[id])
    const start_percent = parsed[id][8];
    const end_percent = parsed[id][9];
    const text = 
      'Attacker: ' + characters[parsed[id][6]] + '\n' +
      'Move : ' + attack_ids[parsed[id][4]] + '\n' +
      'Defender: ' + characters[parsed[id][7]] + '\n' +
      "Start percent: " + start_percent + "\n" +
      "End percent: " + end_percent + "\n";

    document.getElementById("info").innerText = text;
  }
})
