// Replace your-framework with the framework you are using, e.g. react-vite, nextjs, etc.
import { setProjectAnnotations } from '@storybook/nextjs-vite';
import * as previewAnnotations from './preview';

const _annotations = setProjectAnnotations([previewAnnotations]);
