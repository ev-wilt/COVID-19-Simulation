import * as PIXI from 'pixi.js';
const rust = import('./pkg/index');

const healthyColor = 0xadd8e6;
const sickColor = 0xba6d20;
const recoveredColor = 0xa885cc;
const appWidth = 400;
const appHeight = 400;

let simType = 'freeForAll';

function initChart() {
    let ctx = document.getElementById('statusChart').getContext('2d');
}

function getPercentage() {
    const percentageInput = document.getElementById('percentage');
    return parseInt(percentageInput.value);
}

rust.then(m => {
    const app = new PIXI.Application({
        width: appWidth, 
        height: appHeight, 
        backgroundColor: 0xffffff, 
        resolution: window.devicePixelRatio || 1,
        antialias: true
    });
    let simulation = new m.Simulation(appWidth, appHeight, simType, 0);
    let viewContainer = document.createElement('div');
    const graphics = new PIXI.Graphics();
    app.stage.addChild(graphics);

    app.view.setAttribute('style', 'border-style: solid;');
    viewContainer.setAttribute('style', 'text-align: center;');

    viewContainer.appendChild(app.view);
    document.body.appendChild(viewContainer);

    const newSimButton = document.getElementById('newSim');
    newSimButton.addEventListener('click', event => {
        simulation = new m.Simulation(appWidth, appHeight, simType, getPercentage());
    });

    const radioButtons = document.getElementsByName('simType');
    radioButtons.forEach((radio) => {
        radio.onclick = function() {
            const percentageInput = document.getElementById('percentage');
            const label = document.getElementById('percentageLabel');

            if (radio.value === 'distancing' && percentageInput.hidden) {
                percentageInput.hidden = false;
                label.hidden = false;
            }
            else if (radio.value !== 'distancing' && !percentageInput.hidden){
                percentageInput.hidden = true;
                label.hidden = true;
            }

            simType = radio.value;
        }
    });

    function render() {
        graphics.clear();
        simulation.get_updated_people().forEach(person => {
            graphics.lineStyle(0);
            if (person.status === 'Healthy') {
                graphics.beginFill(healthyColor, 1);
            }
            else if (person.status === 'Sick') {
                graphics.beginFill(sickColor, 1);
            }
            else if (person.status === 'Recovered') {
                graphics.beginFill(recoveredColor, 1);
            }
            graphics.drawCircle(person.x, person.y, 5);
            graphics.endFill();
        })
    }

    function update() {
        app.ticker.add((delta) => {
            simulation.update();
            render()

            // Update counters
            document.getElementById('sickCount').innerHTML = simulation.get_sick_total();
            document.getElementById('healthyCount').innerHTML = simulation.get_healthy_total();
            document.getElementById('recoveredCount').innerHTML = simulation.get_recovered_total();
        });        
    }

    update();
});

