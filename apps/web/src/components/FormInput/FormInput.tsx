'use client';

import cns from 'classnames';
import type { FieldError, UseFormRegisterReturn } from 'react-hook-form';
import { Input } from 'rsc-daisyui';

export interface FormInputProps {
	label: string;
	type: string;
	placeholder: string;
	className?: string;
	error?: FieldError;
	register: UseFormRegisterReturn;
	name: string;
}

function FormInput({ label, type, name, placeholder, className, error, register }: FormInputProps) {
	return (
		<>
			<label htmlFor={name} className="fieldset-legend">
				{label}
			</label>
			<Input
				id={name}
				type={type}
				placeholder={placeholder}
				className={cns('input-bordered w-full', className, error && 'input-error')}
				{...register}
			/>
			{error && <span className="label text-error">{error?.message}</span>}
		</>
	);
}

FormInput.displayName = 'FormInput';

export default FormInput;
