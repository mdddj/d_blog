export interface Permission {
    id: number | undefined
    name: string,
    description: string | undefined,
    create_time: string | undefined,
    permission_url: string | undefined,
    type: "URL" | "REGEX" | "PAGE"
    method: string | undefined,
    group: string | undefined,
}

