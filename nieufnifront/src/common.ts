import { API_BASE } from '@/config';
import { slice, split, filter, join } from 'lodash';

export interface NewArticleData {
    username: string;
    password: string;
    articleTitle: string;
}

export interface HttpHeaders {
    [key: string]: string;
}


export interface Article {
    id: string;
    title: string;
    date: string;
    author_name: string;
    rendered_text: string;
    markdown_text: string;
}

export interface ArticleCreated {
    id: string;
    title: string;
    date: string;
    authors_id: number;
    markdown_text: string;
    rendered_text: string;

    username: string;
    password: string;
}

export function emptyArticle(): Article {
    return {
        id: '',
        title: '',
        date: '',
        author_name: '',
        rendered_text: '',
        markdown_text: '',
    };
}

export async function getArticle(id: string): Promise<Article> {
    const response = await fetch(`${API_BASE}/artykuly/${id}`);
    if (!response.ok) { throw new Error(response.statusText); }
    return await response.json();
}

export async function allArticles(): Promise<Article[]> {
    const response = await fetch(`${API_BASE}/artykuly`);
    return await response.json();
}


export async function post(url: string, data: object, username?: string, password?: string) {
    const headers: HttpHeaders = {
        'Content-Type': 'application/json',
    };

    if (username && password) {
        headers['not-trustworthy-username'] = username;
        headers['not-trustworthy-password'] = password;
    }
    const jsonData = JSON.stringify(data);

    fetch(url,
        {
            method: 'POST',
            headers,
            body: jsonData,
        }).then((response) => response.ok)
        // tslint:disable-next-line:no-console
        .catch((e) => console.error(e));
}

export async function sendArticle(
    title: string,
    markdownText: string,
    username: string,
    password: string,
): Promise<boolean> {
    const article: ArticleCreated = {
        title,
        username,
        password,
        markdown_text: markdownText,
        authors_id: 0,  // empty
        id: '',  // empty
        date: '',  // empty
        rendered_text: '',  // empty
    };

    try {
        post('/api/artykuly/', article, username, password);
        return true;
    } catch (e) {
        // tslint:disable-next-line:no-console
        console.error(e);
        return false;
    }
}

export function articleSummary(article: Article): string {
    const description = article.markdown_text;
    const aftertitle = filter(split(description, '\n'))[1];
    return join(slice(aftertitle, 0, 120), '') + '...';
}
