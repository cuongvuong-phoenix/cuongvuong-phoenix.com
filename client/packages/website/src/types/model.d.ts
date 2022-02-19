declare namespace Model {
  export interface PostTag {
    id: string;
    name: string;
    icon?: string;
  }

  export interface PostTagR extends PostTag {
    active?: boolean;
  }

  export interface PostListItem {
    id: string;
    slug: string;
    title: string;
    createdAt: string;
    updatedAt?: string;
    readingTime: number;
    tags: PostTag[];
  }

  export interface PostListItemR extends Omit<PostListItem, 'createdAt' | 'updatedAt' | 'tags'> {
    createdAt: Date;
    updatedAt?: Date;
    tags: PostTagR[];
  }
}
