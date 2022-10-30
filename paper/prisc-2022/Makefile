.PHONY: all
all: prisc-2022

.PHONY: prisc-2022
prisc-2022:
	pdflatex $@
	bibtex $@
	pdflatex $@
	pdflatex $@

.PHONY: clean
clean:
	rm -f *.pdf *.out *.aux *.log *.bbl *.blg
