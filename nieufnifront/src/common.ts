export interface Article {
    id: number;
    date: string;
    author: string;
    markdownText: string;
}

export function getArticle(id: number): Article {
    return {
        id,
        date: `${id}.03.2002`,
        author: 'niedzwiedz',
        markdownText: `#Artykul #${id}\nLorem ipsum dolor sit amet, consectetur adipisicing elit. Beatae ducimus, ex expedita hic inventore placeat praesentium quam tempore! Deserunt dicta fuga inventore laborum maxime minima modi mollitia pariatur recusandae voluptates!`,
    };
}

