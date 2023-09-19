import type { RequestHandler } from './$types';
import { REDIS_URL } from '$env/static/private';
import { createClient as createRedisClient } from '@redis/client';

const redis = () => {
	const client = createRedisClient({
		url: REDIS_URL
	});
	return client.connect().then(() => [client.disconnect.bind(client), client] as const);
};

const TIMEZONE_OFFSET = 2;

function timestamp(date: Date) {
	date.setUTCHours(date.getUTCHours() + TIMEZONE_OFFSET);
	return (
		[
			date.getFullYear(),
			(date.getUTCMonth() + 1).toString().padStart(2, '0'),
			date.getUTCDate().toString().padStart(2, '0')
		].join('-') +
		' ' +
		[
			date.getUTCHours().toString().padStart(2, '0'),
			date.getUTCMinutes().toString().padStart(2, '0')
		].join(':')
	);
}

export const POST: RequestHandler = async ({ url, request, getClientAddress }) => {
	const data = await request.json();

	const addr = getClientAddress();
	if (!addr.startsWith('127.') && !addr.startsWith('::1')) {
		const [close, db] = await redis();
		const res = await db.zAdd(data.problem ? 'rkv-problem' : 'rkv-data', {
			score: Date.now(),
			value: timestamp(new Date()) + ' - ' + addr + ' - ' + data.clipboard
		});
		close();
	}
	return new Response();
};
