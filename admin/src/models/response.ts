
interface ApiResponse<T> {
  code: number
  msg: string
  data: T
}

export {
  ApiResponse
}