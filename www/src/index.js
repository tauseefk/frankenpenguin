import { Frankenpenguin } from "../webgl/webgl.js";

const frankenpenguin = Frankenpenguin.new();
let lastTime = performance.now();
let frameCount = 0;
const fpsElement = document.getElementById("fps");

const render = () => {
  const currentTime = performance.now();
  frameCount++;

  if (currentTime - lastTime >= 1000) {
    fpsElement.textContent = `FPS: ${frameCount}`;
    frameCount = 0;
    lastTime = currentTime;
  }

  frankenpenguin.tick();

  requestAnimationFrame(render);
};
render();
