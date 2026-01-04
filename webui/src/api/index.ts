import { getAlbumDetails, getAlbums, getAlbumsCount, getImageCountInAlbum, getImagesInAlbum } from './albums';
import { login, refreshToken } from './auth';
import {
  getFilteredImageCount,
  getFilteredImages,
  getImage,
  getImageCount,
  getImages,
  getImageUrl,
  getThumbnailUrl,
  setImageTags,
  uploadImageRaw,
} from './images';
import {
  deleteAllTrashedImages,
  deleteTrashedImage,
  getTrashedImageCount,
  getTrashedImages,
  restoreImage,
  trashImage,
} from './trash';
import { getMe } from './user';

export default {
  login,
  refreshToken,

  getMe,

  getImage,
  getImages,
  getImageCount,
  getFilteredImages,
  getFilteredImageCount,
  getImageUrl,
  getThumbnailUrl,
  setImageTags,
  uploadImageRaw,

  getTrashedImages,
  getTrashedImageCount,
  trashImage,
  restoreImage,
  deleteTrashedImage,
  deleteAllTrashedImages,

  getAlbums,
  getAlbumsCount,
  getAlbumDetails,
  getImagesInAlbum,
  getImageCountInAlbum,
};

export type * from './albums';
export type * from './auth';
export type * from './images';
export type * from './pagination';
export type * from './user';
