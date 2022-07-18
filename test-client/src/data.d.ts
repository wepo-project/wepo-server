export interface PostModel {
    content: string
    create_time: string
    id: string
    like_count: number
    hate_count: number
    comment_count: number
    liked: boolean
    hated: boolean
    sender: UserData
    origin_content?: string
    origin_id?: string
    origin_sender?: UserData
    origin_create_time?: string
}

export interface UserData {
    id: string,
    nick: string,
    avatar_url: string
}