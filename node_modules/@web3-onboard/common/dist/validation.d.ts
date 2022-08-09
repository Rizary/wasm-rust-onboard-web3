import Joi from 'joi';
import type { SelectAccountOptions } from './types';
declare type ValidateReturn = Joi.ValidationResult | null;
export declare const validateSelectAccountOptions: (data: SelectAccountOptions) => ValidateReturn;
export {};
