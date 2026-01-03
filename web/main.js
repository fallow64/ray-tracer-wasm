const elements = {
  widthInput: document.getElementById("width-input"),
  heightInput: document.getElementById("height-input"),
  renderButton: document.getElementById("render-button"),
  canvas: document.getElementById("canvas"),
  fpsDisplay: document.getElementById("fps"),
};

// Configuration
const TARGET_FPS = 5; // Set to 0 for vsync
const FRAME_TIME = TARGET_FPS > 0 ? 1000 / TARGET_FPS : 0;
const FPS_UPDATE_INTERVAL = 100;

let width = 400;
let height = 200;
// Holds the current frame request ID or timeout ID
let currentFrameId = null;

/** Loads the WASM module and instanciates it. */
async function loadWasm() {
  const response = await fetch("ray_tracer_wasm.wasm");
  if (!response.ok) {
    throw new Error(`Failed to fetch WASM module: ${response.statusText}`);
  }

  const buffer = await response.arrayBuffer();
  const module = await WebAssembly.compile(buffer);

  const importObject = {
    env: {
      console_log: (ptr, len) => {
        const memory = instance.exports.memory;
        const bytes = new Uint8Array(memory.buffer, ptr, len);
        const text = new TextDecoder().decode(bytes);
        console.log(text);
      },
      console_error: (ptr, len) => {
        const memory = instance.exports.memory;
        const bytes = new Uint8Array(memory.buffer, ptr, len);
        const text = new TextDecoder().decode(bytes);
        console.error(text);
      },
    },
  };

  const instance = await WebAssembly.instantiate(module, importObject);
  return instance;
}

/** Sets up the UI buttons and inputs. */
function setupUIButtons() {
  elements.widthInput.value = width;
  elements.heightInput.value = height;
  elements.renderButton.onclick = async () => {
    width = parseInt(elements.widthInput.value) || 256;
    height = parseInt(elements.heightInput.value) || 256;
    await setup(width, height);
  };
}

/** Sets up the canvas logic to render from WASM memory. */
function setupCanvas(width, height, instance, rendererTarget) {
  const { memory, render } = instance.exports;

  elements.canvas.width = width;
  elements.canvas.height = height;
  const ctx = elements.canvas.getContext("2d");
  const imageData = ctx.createImageData(width, height);

  let lastTime = performance.now();
  let frameCount = 0;

  /** Schedules the next frame render based on TARGET_FPS. */
  function scheduleNextFrame() {
    if (TARGET_FPS > 0) {
      currentFrameId = setTimeout(renderFrame, FRAME_TIME);
    } else {
      currentFrameId = requestAnimationFrame(renderFrame);
    }
  }

  /** Renders a single frame from WASM memory to the canvas. */
  function renderFrame() {
    // Call the WASM render function
    render(rendererTarget);

    // FPS calculation
    frameCount++;
    const now = performance.now();
    const elapsed = now - lastTime;
    if (elapsed >= FPS_UPDATE_INTERVAL) {
      const fps = Math.round(frameCount * (1000 / elapsed));
      elements.fpsDisplay.textContent = `FPS: ${fps}`;
      frameCount = 0;
      lastTime = now;
    }

    // dataPtr: *PixelArray = *targetPtr
    const dataPtr = new Uint32Array(memory.buffer, rendererTarget, 1)[0];
    // pixelData: [u8] RGBA = *dataPtr (4 bytes per pixel)
    const pixelData = new Uint8ClampedArray(
      memory.buffer,
      dataPtr,
      width * height * 4
    );

    imageData.data.set(pixelData);

    ctx.putImageData(imageData, 0, 0);
    scheduleNextFrame();
  }

  scheduleNextFrame();
}

async function setup(width, height) {
  // Cancel previous render loop
  if (currentFrameId !== null) {
    if (TARGET_FPS > 0) {
      clearTimeout(currentFrameId);
    } else {
      cancelAnimationFrame(currentFrameId);
    }
    currentFrameId = null;
  }

  const instance = await loadWasm();

  console.log("Creating renderer target...");
  const { create_renderer_target } = instance.exports;
  const targetPtr = create_renderer_target(width, height);
  console.log("Setting up canvas...");
  setupCanvas(width, height, instance, targetPtr);
}

/** Main entry point. */
async function main() {
  setupUIButtons();
  await setup(width, height);
}

main();
