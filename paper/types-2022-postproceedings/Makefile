.PHONY: all
all: types-2022-postproceedings

.PHONY: types-2022-postproceedings
types-2022-postproceedings:
	pdflatex $@
	bibtex $@
	pdflatex $@
	pdflatex $@

.PHONY: clean
clean:
	rm -f *.pdf *.out *.aux *.log *.bbl *.blg
