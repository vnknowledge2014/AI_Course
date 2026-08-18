import { Application, Graphics, Container, Text, TextStyle } from 'pixi.js';

export class LiveFactory {
  private app: Application;
  private container: Container;
  private cubes: Container[] = [];
  private gears: Graphics[] = [];
  private beltOffset: number = 0;
  private beltGraphics: Graphics;
  private isProcessing: boolean = false;
  private statusText: Text;
  private destroyed: boolean = false;
  private animations: { type: 'machine', name: string, in: string, out: string, progress: number }[] = [];
  
  constructor(container: HTMLElement) {
    this.app = new Application();
    this.container = new Container();
    this.beltGraphics = new Graphics();
    
    // We add status text to show what's happening
    const style = new TextStyle({
      fontFamily: 'JetBrains Mono',
      fontSize: 16,
      fill: '#636366',
      fontWeight: 'bold',
      letterSpacing: 2
    });
    this.statusText = new Text({ text: 'IDLE', style });
    
    // Launch async init but don't block constructor
    this.init(container).catch(console.error);
  }

  private async init(container: HTMLElement) {
    await this.app.init({
      resizeTo: container,
      backgroundColor: 0x1e1e2e,
      antialias: true
    });

    if (this.destroyed) {
      // It was destroyed before init finished (React StrictMode behavior)
      this.app.destroy(true, { children: true });
      return;
    }

    container.appendChild(this.app.canvas);

    this.app.stage.addChild(this.container);
    
    // Draw Environment
    this.drawBackground();
    this.container.addChild(this.beltGraphics); // Belt
    this.drawGears();
    
    // Status text
    this.statusText.x = 20;
    this.statusText.y = 40;
    this.container.addChild(this.statusText);

    // Initial Cubes
    this.spawnCubes();

    // Game Loop
    this.app.ticker.add((ticker) => {
      this.update(ticker.deltaTime);
    });
  }

  private drawBackground() {
    const bg = new Graphics();
    bg.rect(0, 0, 2000, 2000);
    bg.fill(0x1e1e2e);
    this.container.addChild(bg);
    
    // Draw some factory pipes
    const pipe = new Graphics();
    pipe.rect(0, 60, 2000, 20);
    pipe.fill(0x2c2c2e);
    pipe.rect(100, 0, 30, 2000);
    pipe.fill(0x2c2c2e);
    this.container.addChild(pipe);
  }

  private drawGears() {
    for(let i=0; i<4; i++) {
      const gear = new Graphics();
      gear.poly([
        0,-15, 5,-15, 5,-5, 15,-5, 15,5, 5,5, 5,15, 0,15,
        -5,15, -5,5, -15,5, -15,-5, -5,-5, -5,-15
      ]);
      gear.fill(0x4a4a4c);
      
      gear.x = 80 + i * 150;
      gear.y = this.app.screen.height / 2 + 50;
      this.gears.push(gear);
      this.container.addChild(gear);
    }
  }

  private drawConveyorBelt() {
    const width = this.app.screen.width || 1000;
    const height = this.app.screen.height || 1000;
    const beltY = height / 2;
    
    this.beltGraphics.clear();
    
    // Belt Base
    this.beltGraphics.rect(0, beltY, width, 40);
    this.beltGraphics.fill(0x3a3a3c);
    
    // Belt lines (moving)
    for(let x = (this.beltOffset % 40) - 40; x < width; x += 40) {
      this.beltGraphics.moveTo(x, beltY);
      this.beltGraphics.lineTo(x, beltY + 40);
    }
    this.beltGraphics.stroke({ width: 2, color: 0x1e1e2e });
  }

  private spawnCubes() {
    const beltY = (this.app.screen.height || 500) / 2;
    for (let i = 0; i < 3; i++) {
      const cube = new Container();
      
      const box = new Graphics();
      box.roundRect(-20, -20, 40, 40, 6);
      box.fill(0x0a84ff);
      
      // Data label
      const label = new Text({ text: `d${i}`, style: { fontSize: 12, fill: '#ffffff', fontFamily: 'monospace' }});
      label.anchor.set(0.5);

      cube.addChild(box, label);
      cube.x = 100 + i * 100;
      cube.y = beltY - 20;
      
      this.cubes.push(cube);
      this.container.addChild(cube);
    }
  }

  private update(dt: number) {
    if (!this.app.screen) return;
    const width = this.app.screen.width || 1000;
    
    // Idle animation: gears rotate slowly, belt moves slowly
    const speed = this.isProcessing ? 3 : 0.5;
    
    this.gears.forEach((g, i) => {
      g.rotation += (i % 2 === 0 ? 0.02 : -0.02) * speed * dt;
    });

    this.beltOffset += speed * dt;
    this.drawConveyorBelt();

    // Floating cubes if idle
    if (!this.isProcessing) {
      this.cubes.forEach((cube, i) => {
        const time = performance.now();
        cube.y = (this.app.screen.height / 2 - 20) + Math.sin(time / 500 + i) * 3;
      });
    } else {
      this.cubes.forEach((cube) => {
        // Basic movement
        if (this.isProcessing && cube.x < width) {
          cube.x += speed * dt;
        }
      });
    }

    // Handle process animations
    for (let i = this.animations.length - 1; i >= 0; i--) {
      const anim = this.animations[i];
      anim.progress += dt * 0.02; // Roughly 1 second

      // If animation completes, we can remove it or keep it at the end
      if (anim.progress >= 1) {
        this.animations.splice(i, 1);
      }
    }
  }

  public animateProcess(machineName: string, inputVal: string, outputVal: string) {
    this.isProcessing = true;
    
    // Log for debugging
    console.log(`Animating machine: ${machineName} with input ${inputVal} -> ${outputVal}`);
    const machine = new Graphics();
    machine.rect(0, 0, 100, 100);
    machine.fill(0xffaa00);
    machine.x = this.app.screen.width / 2 - 50;
    machine.y = this.app.screen.height / 2 - 50;
    this.container.addChild(machine);

    // In a real game, this would be an animated sequence.
    // For this prototype, we'll just show the machine, then after 1s show the output box
    setTimeout(() => {
        machine.clear();
        machine.rect(0, 0, 100, 100);
        machine.fill(0x30d158); // Turn green
        setTimeout(() => {
            this.container.removeChild(machine);
            this.isProcessing = false;
        }, 1000);
    }, 1000);
  }

  public playSuccess() {
    this.isProcessing = true;
    this.statusText.text = 'PROCESSING: SUCCESS';
    this.statusText.style.fill = '#30d158';

    // Move cubes forward and turn them into green spheres
    this.cubes.forEach((cube, i) => {
      const box = cube.children[0] as Graphics;
      setTimeout(() => {
        box.clear();
        box.circle(0, 0, 22);
        box.fill(0x30d158);
        
        // Simple animation
        let tick = 0;
        const animate = () => {
          if (tick > 30) return;
          cube.x += 8;
          cube.rotation += 0.1;
          tick++;
          requestAnimationFrame(animate);
        };
        animate();
      }, i * 300);
    });

    setTimeout(() => { 
      this.isProcessing = false; 
      this.statusText.text = 'IDLE';
      this.statusText.style.fill = '#636366';
    }, 2000);
  }

  public playError() {
    this.isProcessing = true;
    this.statusText.text = 'ERROR: HALTED';
    this.statusText.style.fill = '#ff453a';

    // Shake screen and turn cubes red
    let shakeCount = 0;
    const shake = () => {
      if (shakeCount > 20) {
        this.container.x = 0;
        this.container.y = 0;
        return;
      }
      this.container.x = (Math.random() - 0.5) * 15;
      this.container.y = (Math.random() - 0.5) * 15;
      shakeCount++;
      requestAnimationFrame(shake);
    };
    shake();

    this.cubes.forEach(cube => {
      const box = cube.children[0] as Graphics;
      box.clear();
      box.drawRoundedRect(-22, -22, 44, 44, 4);
      box.fill(0xff453a);
    });

    setTimeout(() => { 
      this.isProcessing = false; 
      this.statusText.text = 'SYSTEM FAULT';
    }, 1000);
  }

  public destroy() {
    this.destroyed = true;
    try {
      this.app.destroy(true, { children: true });
    } catch(e) {}
  }
}
