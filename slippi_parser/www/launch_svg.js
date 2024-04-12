// Fetch the given SVG file and load it into the page.
var svg = document.getElementById("svgobject");

svg.addEventListener("load", function(){
  var svgDoc = svg.contentDocument.querySelectorAll('svg');
  var nodes = svgDoc[0].querySelectorAll('path');

  for( var i = 0; i < nodes.length; i++ ) {
    nodes[i].addEventListener('click', changeStroke);
    //TODO: add listener to reset strokecolor.
  }
  console.log(parsed);
  var lastPathClicked = null;

  panzoom = Panzoom(svg);
  if(panzoom)
    console.log("Panzoom successfully initialized.");
  svg.parentElement.addEventListener('wheel', panzoom.zoomWithWheel);

  function changeStroke() {
    this.setAttribute('opacity', 0.30);
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
      "Start percent: " + start_percent + "\n" +
      "End percent: " + end_percent + "\n";

    document.getElementById("info").innerText = text;
  }
})
