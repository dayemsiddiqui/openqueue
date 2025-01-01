import http from 'k6/http';
import { check, sleep } from 'k6';

export const options = {
    stages: [
        { duration: '5s', target: 100 },
        // { duration: '30s', target: 100 },
        // { duration: '5s', target: 1000 },
        // { duration: '30s', target: 1000 },
        // { duration: '5s', target: 0 },
    ]
}

export default async function () {
    // Randomly choose between publish and consume with equal probability
    if (Math.random() < 0.5) {
        await callPublish();
    } else {
        await consumeMessage();
    }
}

const generateMessage = () => {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
}

const callPing = async () => {
    const res = await http.get(`http://localhost:3000/ping`);
    check(res, {
        'status is 200': (r) => r.status === 200,
    });
}

const consumeMessage = async () => {
    const res = await http.get(`http://localhost:3000/consume`);

    const messageId = res.json().message.id;
    check(res, {
        'status is 200': (r) => r.status === 200,
        'body status is success': (r) => r.json().status === 'success',
        'body message is not empty': (r) => r.json().message.id === messageId,
    });

    // Sleep for 1 second to simulate processing time
    sleep(1);

    // Acknowledge the message
    const ackRes =  await http.post(`http://localhost:3000/ack`, {
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
            queue: 'test_queue',
            messageId: messageId,
        }),
    });

    check(ackRes, {
        'status is 200': (r) => r.status === 200,
        'body status is success': (r) => r.json().status === 'success',
    });

    
}   

const callPublish = async () => {
    const queue = 'test_queue';
    const message = generateMessage();
    const payload = JSON.stringify({
        queue: queue,
        message: message,
    });
    const params = {
        headers: { 'Content-Type': 'application/json' },
    };
    try {
        const res = await http.post(`http://localhost:3000/publish`, payload, params);
        check(res, {
            'status is 200': (r) => r.status === 200,
            'body status is success': (r) => r.json().status === 'success',
        });
    } catch (e) {
        console.log(e);
    }
}