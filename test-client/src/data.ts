interface PostModel {
    comment_count: number
    content: string
    create_time: string
    id: string
    like_count: number
    liked: boolean
    sender: PostSender
    origin_content?: string
    origin_id?: string
    origin_sender?: PostSender
    origin_create_time?: string
}

interface PostSender {
    id: string,
    nick: string,
    avatar_url: string
}