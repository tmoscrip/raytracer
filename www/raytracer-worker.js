// Web Worker for tile-based raytracing
importScripts("./pkg/raytracer.js");

let wasmModule = null;
let renderContext = null;
let memory = null;
let RenderContext = null;

// Initialize the WASM module
async function initWasm() {
  try {
    // Use the modern initialization approach
    const exports = await wasm_bindgen({
      module_or_path: "./pkg/raytracer_bg.wasm",
    });

    wasmModule = exports;
    memory = wasmModule.memory;

    // Get the RenderContext constructor from wasm_bindgen namespace
    RenderContext = wasm_bindgen.RenderContext;

    if (!RenderContext) {
      throw new Error("RenderContext not found in WASM exports");
    }

    // Send ready signal to main thread
    self.postMessage({ type: "ready" });
  } catch (error) {
    self.postMessage({ type: "error", error: error.message });
  }
}

// Handle messages from main thread
self.onmessage = async function (e) {
  const { type, data } = e.data;

  switch (type) {
    case "init":
      await initWasm();
      break;

    case "render-tile":
      if (!wasmModule) {
        self.postMessage({ type: "error", error: "WASM not initialized" });
        return;
      }

      try {
        const {
          tileId,
          tileX,
          tileY,
          tileWidth,
          tileHeight,
          fullWidth,
          fullHeight,
          lightX,
          lightY,
          lightZ,
          sphereX,
          sphereY,
          sphereZ,
        } = data;

        // Create or reuse render context
        if (!renderContext) {
          renderContext = new RenderContext(fullWidth, fullHeight);
        }

        // Update scene parameters
        renderContext.update_light_position(lightX, lightY, lightZ);
        renderContext.update_sphere_position(sphereX, sphereY, sphereZ);

        // Render the tile and store it
        renderContext.render_tile_and_store(
          tileX,
          tileY,
          tileWidth,
          tileHeight,
          fullWidth,
          fullHeight
        );

        // Get the tile data from WASM memory
        const tileBufferPointer = renderContext.get_tile_buffer_pointer();
        const tileBufferSize = renderContext.get_tile_buffer_size();

        const tileData = new Uint8ClampedArray(
          memory.buffer,
          tileBufferPointer,
          tileBufferSize
        );

        // Copy the data to avoid memory issues
        const tileDataCopy = new Uint8ClampedArray(tileData);

        // Send the rendered tile back to main thread
        self.postMessage(
          {
            type: "tile-complete",
            data: {
              tileId,
              tileX,
              tileY,
              tileWidth,
              tileHeight,
              tileData: tileDataCopy,
            },
          },
          [tileDataCopy.buffer]
        );
      } catch (error) {
        self.postMessage({
          type: "error",
          error: error.message,
          tileId: data.tileId,
        });
      }
      break;

    default:
      self.postMessage({
        type: "error",
        error: `Unknown message type: ${type}`,
      });
  }
};

// Initialize immediately
self.postMessage({ type: "worker-started" });
