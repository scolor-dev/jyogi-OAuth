type ErrorMessageProps = {
  message?: string
}

export default function ErrorMessage({ message }: ErrorMessageProps) {
  // undefined および空文字列はエラーなしとして扱う
  if (message == null || message === '') return null
  return <p className="text-sm text-red-600">{message}</p>
}
