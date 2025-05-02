export const ERRNO: Record<number, string> = {
  5010: '找不到指定 ID 的设备',
  5011: '设备没有公开的心率服务'
}

export const useErrno = (errno: string | null | undefined) => {
  if (errno) {
    const err = ERRNO[Number(errno)]
    if (err) {
      return err
    }
  }
  return '未知错误'
}