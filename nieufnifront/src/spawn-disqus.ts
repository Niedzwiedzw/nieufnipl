/* tslint:disable */
export function spawnDisqus(articleId: string) {
    const url = `http://www.nieufni.pl/${articleId}`;
    const identifier = articleId;

    console.info('spawning Disqus');
    console.info('url:', url);
    console.info('identifier: ', identifier);

    (window as any).disqus_config = function() {
        this.page.url = url;
        this.page.identifier = identifier;
    };
    setTimeout(() => {
        const s = document.createElement('script');
        s.src = 'https://nieufni.disqus.com/embed.js';
        s.setAttribute('data-timestamp', `${new Date()}`);
        s.setAttribute('id', 'embed-disqus');
        s.type = 'text/javascript';
        s.async = true;
        document.body.appendChild(s);
    }, 5000);
}
