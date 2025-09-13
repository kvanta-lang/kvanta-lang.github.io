import {createTheme} from 'thememirror';
import {tags as t} from '@lezer/highlight';

export const quantaTheme = createTheme({
	variant: 'dark',
	settings: {
		background: '#22272e',
		foreground: '#adbac7',
		caret: '#539bf5',
		selection: '#1a5fb4',
		lineHighlight: '#8a91991a',
		gutterBackground: '#22272e',
		gutterForeground: '#8a919966',
	},
	styles: [
		{
			tag: t.comment,
			color: '#787b8099',
		},
		{
			tag: t.variableName,
			color: '#D3C6AA',
		},
		{
			tag: [t.string, t.special(t.brace)],
			color: '#D699B6',
		},
		{
			tag: t.number,
			color: '#D699B6',
		},
		{
			tag: t.bool,
			color: '#D699B6',
		},
		{
			tag: t.null,
			color: '#8deedeff',
		},
		{
			tag: t.keyword,
			color: '#E67E80',
		},
        {
            tag: t.function(t.variableName),
            color: '#A7C080',
        },
        {
            tag: t.paren,
            color: '#ffea00ff',
        },
		{
			tag: t.operator,
			color: '#ffbe6f',
		},
		{
			tag: t.moduleKeyword,
			color: '#80C080',
		},
		{
			tag: t.definition(t.typeName),
			color: '#83c092',
		},
		{
			tag: t.typeName,
			color: '#7FBBB3',
		},
        {
			tag: t.definitionKeyword,
			color: '#7FBBB3',
		},
		{
			tag: t.angleBracket,
			color: '#f9f06b',
		},
		{
			tag: t.tagName,
			color: '#5e5c64',
		},
		{
			tag: t.attributeName,
			color: '#5c6166',
		},
	],
});