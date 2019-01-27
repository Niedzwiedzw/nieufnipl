import { Metatag } from '@/common';
const DEFAULT_META: [string, string, string, string, string] = [
    'website',
    'https://i.ytimg.com/vi/W9t6GZ0vNPA/hqdefault.jpg',
    'nieufni.pl - podejrzliwa strona Internetu',
    'http://nieufni.pl/',
    'Newsy, Artykuły, Kultura, Sport, Kuchnia, Muzyka.',
];
export function setMeta(type: string, image: string, title: string, url: string, description: string) {
    const tags = [
        new Metatag('type', type),
        new Metatag('image', image),
        new Metatag('title', title),
        new Metatag('url', url),
        new Metatag('description', description),
    ];
    document.title = title;
    for (const metatag of tags) {
        metatag.inject();
    }
}

export function setDefaultMeta() {
    setMeta(...DEFAULT_META);
}

export const API_BASE = '/api';
