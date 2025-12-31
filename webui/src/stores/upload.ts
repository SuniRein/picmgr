import api from '@/api';

export interface UploadTask {
  file: File;
  preview: string;
  progress: number;
  status: 'idle' | 'uploading' | 'success' | 'error';
}

export const useUploadStore = defineStore('upload', () => {
  const tasks = ref<UploadTask[]>([]);
  const idleTasksCount = computed(() => tasks.value.filter(t => t.status === 'idle').length);

  function addTask(file: File) {
    tasks.value.push({
      file,
      preview: URL.createObjectURL(file),
      progress: 0,
      status: 'idle',
    });
  }

  function removeTask(index: number) {
    const task = tasks.value[index];
    if (task) {
      URL.revokeObjectURL(task.preview);
      tasks.value.splice(index, 1);
    }
  }

  function clearCompleted() {
    tasks.value.forEach((task) => {
      if (task.status === 'success')
        URL.revokeObjectURL(task.preview);
    });
    tasks.value = tasks.value.filter(task => task.status !== 'success');
  }

  async function processQueue() {
    const idleTasks = tasks.value.filter(t => t.status === 'idle');
    await Promise.all(idleTasks.map(task => uploadFile(task)));
  }

  async function uploadFile(task: UploadTask) {
    task.status = 'uploading';
    try {
      await api.uploadImageRaw(task.file, percent => task.progress = percent);
      task.status = 'success';
    }
    catch (e) {
      task.status = 'error';
      console.error('上传失败:', e);
    }
  }

  return {
    tasks,
    idleTasksCount,
    addTask,
    removeTask,
    clearCompleted,
    processQueue,
  };
});
