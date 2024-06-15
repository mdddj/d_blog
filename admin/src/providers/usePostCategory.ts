import { create } from 'zustand';
import { PostCategory } from '@/models/post';
import axiosInstance from '@/utils/request';
import { ApiResponse } from '@/models/response';

type PostCategoryProp = {
  data: PostCategory[],
  fetch: () => Promise<void>,
  add: (n: PostCategory) => void
}

export const usePostCategory = create<PostCategoryProp>((set) => ({
  data: [],
  fetch: async () => {
    const response = await axiosInstance.get<ApiResponse<PostCategory[]>>('/api/category');
    if (response.status === 200) {
      set({ data: response.data.data });
    }
  },
  add: function(n: PostCategory) {
    set((state) => ({ ...state, data: [...state.data, n] }));
  },
}));
