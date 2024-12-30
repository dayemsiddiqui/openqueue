import http from 'k6/http';
import { check } from 'k6';

export const options = {
    stages: [
        { duration: '5s', target: 100 },
        { duration: '5s', target: 1000 },
        { duration: '5s', target: 10000 },
        { duration: '5s', target: 100000 },
    ],
}

export default async function () {
    // callPing();
    callPublish();
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