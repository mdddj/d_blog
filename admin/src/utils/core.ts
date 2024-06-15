///显示dialog
import { Renderable, ValueOrFunction } from 'react-hot-toast';

export function showDialogById(id: string) {
  const ele = document.getElementById(id);
  if (ele !== undefined) {
    (ele as HTMLDialogElement).showModal();
  }
}

///关闭dialog
export function closeDialogById(id: string) {
  const ele = document.getElementById(id);
  if (ele != null) {
    (ele as HTMLDialogElement).close();
  }
}

export function defaultToastMsg(success?:()=>void): {
  loading: Renderable;
  success: ValueOrFunction<Renderable, any>;
  error: ValueOrFunction<Renderable, any>;
} {
  return {
    loading: '正在处理',
    success: () => {
      success?.()
      return '处理成功'
    },
    error: arg => `处理失败:${arg}`,
  };
}


export class MyDialog {
  id: string
  constructor(id: string) {
    this.id = id
  }

  show = () => {
    showDialogById(this.id)
  }

  close = () => {
    closeDialogById(this.id)
  }
}