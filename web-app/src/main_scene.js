import { Scene } from 'phaser';

export default class MainScene extends Scene {
  static new(socket) {
    const self = new MainScene();
    self.id = null;
    self.players = new Map();
    self.state = {};
    self.socket = socket;

    socket.onmessage = (m) => {
      let message = JSON.parse(m.data);
      console.log(message);
      if (message == 'identify') {
        self.id = Math.round(Math.random() * 100000);
        socket.send(JSON.stringify({ identify: self.id }));
      } else if (message['state']) {
        self.state = message['state'];
      }
    };

    return self;
  }
  preload() {
    this.load.image('player', 'player.png');
  }

  update() {
    for (let player in this.state.players) {
      let [x, y] = this.state.players[player].position;
      if (!this.players.has(player)) {
        this.players.set(player, this.add.sprite(x, y, 'player'));
      }

      let sprite = this.players.get(player);
      sprite.x = x;
      sprite.y = y;
    }

    let worldX = this.input.mousePointer.worldX;
    let worldY = this.input.mousePointer.worldY;

    this.state.players[this.id.toString()].position.x = worldX;
    this.state.players[this.id.toString()].position.y = worldY;

    const sprite = this.players.get(this.id.toString());
    sprite.x = worldX;
    sprite.y = worldY;

    this.socket.send(JSON.stringify({ new_position: [worldX, worldY] }));
  }
}
