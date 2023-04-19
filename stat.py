import time
init = {
    'Planet': {'time': 0, 'timeStep': 0.01, 'x': 0, 'y': 0.1, 'vx': 0.1, 'vy': 0},
    'Satellite': {'time': 0, 'timeStep': 0.01, 'x': 0, 'y': 1, 'vx': 1, 'vy': 0},
}

def propagate(agentId, universe):
    state = universe[agentId]
    time, timeStep, x, y, vx, vy = state['time'], state['timeStep'], state['x'], state['y'], state['vx'], state['vy']

    if agentId == 'Planet':
        x += vx * timeStep
        y += vy * timeStep
    elif agentId == 'Satellite':
        px, py = universe['Planet']['x'], universe['Planet']['y']
        dx = px - x
        dy = py - y
        fx = dx / (dx**2 + dy**2)**(3/2)
        fy = dy / (dx**2 + dy**2)**(3/2)
        vx += fx * timeStep
        vy += fy * timeStep
        x += vx * timeStep
        y += vy * timeStep

    return {'time': time + timeStep, 'timeStep': 0.01*0.09, 'x': x, 'y': y, 'vx': vx, 'vy': vy}

start_time = time.time()
ret = propagate("Planet", init)
end_time = time.time()
total = end_time - start_time
print("Elapsed time: ", total) 


