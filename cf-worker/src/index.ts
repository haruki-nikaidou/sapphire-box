import callPolygon from "./polygonCall";

addEventListener('fetch', (event: FetchEvent) => {
    event.respondWith(handleRequest(event.request));
});

async function handleRequest(request: Request): Promise<Response> {
    const url = new URL(request.url);
    const pathSegments = url.pathname.split('/').filter(Boolean);
    const contractParam = pathSegments[0];

    if (!contractParam) {
        return new Response('No contract parameter found.', { status: 400 });
    }

    try {
        const contractData = await callPolygon(contractParam);
        return new Response(contractData, {
            headers: { 'Content-Type': 'application/json' },
        });
    } catch {
        return new Response(`Error: Contract Error`, { status: 500 });
    }
}