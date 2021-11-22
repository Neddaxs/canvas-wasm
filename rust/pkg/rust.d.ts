/* tslint:disable */
/* eslint-disable */
/**
* @param {number} num1
* @param {number} num2
* @returns {number}
*/
export function add(num1: number, num2: number): number;
/**
* @param {string} canvas_id
*/
export function snake(canvas_id: string): void;
/**
*/
export class TannerClient {
  free(): void;
/**
* @returns {TannerClient}
*/
  static new(): TannerClient;
/**
* @param {number} _time
* @param {number} _height
* @param {number} _width
*/
  update(_time: number, _height: number, _width: number): void;
/**
*/
  render(): void;
}
