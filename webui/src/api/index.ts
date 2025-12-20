import { login } from './auth';
import { getImageData, getImagesCount } from './images';
import { getMe } from './user';

export default { login, getMe, getImageData, getImagesCount };

export type * from './auth';
export type * from './images';
export type * from './pagination';
export type * from './user';
