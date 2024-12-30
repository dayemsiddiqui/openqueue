import http from 'k6/http';
import { check } from 'k6';

export const options = {
    vus: 1,
    duration: '1s',
}

export default async function () {
    const queue = 'test_queue';
    const message = generateMessage();
    const payload = JSON.stringify({
        queue: queue,
        message: message,
    });
    const params = {
        headers: {
            'Content-Type': 'application/json',
        },
    };
    const res = await http.post(`http://localhost:3000/publish`, payload, params);
    check(res, {
        'status is 200': (r) => r.status === 200,
        'body status is success': (r) => r.json().status === 'success',
    });
}

const generateMessage = () => {
    return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
}