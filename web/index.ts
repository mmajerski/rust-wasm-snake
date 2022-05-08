import init, { World, Direction, GameStatus } from "snake_game";
import { random } from "./utils/random";

init().then((wasm) => {
  const CELL_SIZE = 20;
  const WORLD_WIDTH = 24;
  const SNAKE_SPAWN_IDX = random(WORLD_WIDTH * WORLD_WIDTH);

  const world = World.new(WORLD_WIDTH, SNAKE_SPAWN_IDX);
  const worldWidth = world.width();

  const gamePlayBtn = document.getElementById("game-play-btn");
  const gamePauseBtn = document.getElementById("game-pause-btn");
  const gameStatusTxt = document.getElementById("game-status");

  const canvas = <HTMLCanvasElement>document.getElementById("snake-canvas");
  const ctx = canvas.getContext("2d");

  canvas.height = worldWidth * CELL_SIZE;
  canvas.width = worldWidth * CELL_SIZE;

  gamePlayBtn.addEventListener("click", () => {
    world.start_game();
  });

  gamePauseBtn.addEventListener("click", () => {
    world.pause_game();
  });

  document.addEventListener("keydown", (e) => {
    switch (e.code) {
      case "ArrowUp":
        world.change_snake_direction(Direction.Up);
        break;
      case "ArrowRight":
        world.change_snake_direction(Direction.Right);
        break;
      case "ArrowDown":
        world.change_snake_direction(Direction.Down);
        break;
      case "ArrowLeft":
        world.change_snake_direction(Direction.Left);
        break;
    }
  });

  function drawWorld() {
    ctx.beginPath();

    for (let x = 0; x < worldWidth + 1; x++) {
      ctx.moveTo(CELL_SIZE * x, 0);
      ctx.lineTo(CELL_SIZE * x, worldWidth * CELL_SIZE);
    }

    for (let y = 0; y < worldWidth + 1; y++) {
      ctx.moveTo(0, CELL_SIZE * y);
      ctx.lineTo(worldWidth * CELL_SIZE, CELL_SIZE * y);
    }

    ctx.strokeStyle = "#a9a9a9";
    ctx.stroke();
  }

  function drawFood() {
    const idx = world.snake_food_cell();
    const col = idx % worldWidth;
    const row = Math.floor(idx / worldWidth);

    ctx.beginPath();
    ctx.fillStyle = "#e1ad01";
    ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);

    ctx.stroke();
  }

  function drawSnake() {
    const snakeCells = new Uint32Array(
      wasm.memory.buffer,
      world.snake_cells(),
      world.snake_length()
    );

    snakeCells
      .slice()
      .reverse()
      .forEach((cellIdx, i) => {
        const col = cellIdx % worldWidth;
        const row = Math.floor(cellIdx / worldWidth);

        ctx.beginPath();
        ctx.fillStyle = i === snakeCells.length - 1 ? "#7c0c7c" : "#2db83d";
        ctx.fillRect(col * CELL_SIZE, row * CELL_SIZE, CELL_SIZE, CELL_SIZE);
      });

    ctx.stroke();
  }

  function drawGameStatus() {
    gameStatusTxt.textContent = world.game_status_text();
  }

  function paint() {
    drawWorld();
    drawSnake();
    drawFood();
    drawGameStatus();
  }

  function update() {
    const fps = 5;
    setTimeout(() => {
      ctx.clearRect(0, 0, canvas.width, canvas.height);
      world.step();
      paint();
      requestAnimationFrame(update);
    }, 1000 / fps);
  }

  update();
});
